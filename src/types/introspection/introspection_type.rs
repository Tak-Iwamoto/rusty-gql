use crate::{
    types::GqlValueType, FieldContext, FieldResolver, GqlTypeDefinition, GqlValue, ResolverResult,
    Schema, SelectionSetContext, SelectionSetResolver,
};

use super::{enum_value::__EnumValue, field::__Field, input_value::__InputValue};

enum TypeDetail<'a> {
    Named(&'a GqlTypeDefinition),
    NonNull(&'a str),
    List(&'a str),
}

pub struct __Type<'a> {
    schema: &'a Schema,
    detail: TypeDetail<'a>,
}

#[allow(non_camel_case_types)]
pub(crate) enum __TypeKind {
    Scalar,
    Object,
    Interface,
    Union,
    Enum,
    InputObject,
    List,
    NonNull,
}

impl ToString for __TypeKind {
    fn to_string(&self) -> String {
        match self {
            __TypeKind::Scalar => "SCALAR".to_string(),
            __TypeKind::Object => "OBJECT".to_string(),
            __TypeKind::Interface => "INTERFACE".to_string(),
            __TypeKind::Union => "UNION".to_string(),
            __TypeKind::Enum => "ENUM".to_string(),
            __TypeKind::InputObject => "INPUT_OBJECT".to_string(),
            __TypeKind::List => "LIST".to_string(),
            __TypeKind::NonNull => "NON_NULL".to_string(),
        }
    }
}

impl<'a> __Type<'a> {
    pub fn from_type_definition(
        schema: &'a Schema,
        type_definition: &'a GqlTypeDefinition,
    ) -> Self {
        __Type {
            schema,
            detail: TypeDetail::Named(type_definition),
        }
    }

    pub fn from_value_type(schema: &'a Schema, value_type: &'a GqlValueType) -> Self {
        let detail = match value_type {
            GqlValueType::NamedType(named) => {
                let type_def = schema.type_definitions.get(named);
                match type_def {
                    Some(def) => TypeDetail::Named(def),
                    None => panic!("Unknown type: '{}'", named),
                }
            }
            GqlValueType::ListType(list) => TypeDetail::List(list.name()),
            GqlValueType::NonNullType(non_null) => TypeDetail::NonNull(non_null.name()),
        };
        __Type { schema, detail }
    }

    async fn kind(&self) -> __TypeKind {
        match self.detail {
            TypeDetail::Named(def) => match def {
                GqlTypeDefinition::Scalar(_) => __TypeKind::Scalar,
                GqlTypeDefinition::Object(_) => __TypeKind::Object,
                GqlTypeDefinition::Interface(_) => __TypeKind::Interface,
                GqlTypeDefinition::Union(_) => __TypeKind::Union,
                GqlTypeDefinition::Enum(_) => __TypeKind::Enum,
                GqlTypeDefinition::InputObject(_) => __TypeKind::InputObject,
            },
            TypeDetail::NonNull(_) => __TypeKind::NonNull,
            TypeDetail::List(_) => __TypeKind::List,
        }
    }

    async fn name(&self) -> Option<&str> {
        match self.detail {
            TypeDetail::Named(def) => Some(def.name()),
            TypeDetail::NonNull(_) => None,
            TypeDetail::List(_) => None,
        }
    }

    async fn description(&self) -> Option<&String> {
        match self.detail {
            TypeDetail::Named(def) => def.description().as_ref(),
            TypeDetail::NonNull(_) => None,
            TypeDetail::List(_) => None,
        }
    }

    async fn fields(&self) -> Option<Vec<__Field<'a>>> {
        if let TypeDetail::Named(def) = self.detail {
            match def.fields() {
                Some(fields) => {
                    let result = fields
                        .into_iter()
                        .map(|field| __Field::new(self.schema, field.clone()))
                        .collect();
                    Some(result)
                }
                None => None,
            }
        } else {
            None
        }
    }

    async fn interfaces(&self) -> Option<Vec<__Type<'a>>> {
        if let TypeDetail::Named(def) = self.detail {
            if let GqlTypeDefinition::Object(obj) = def {
                let mut interfaces = Vec::new();

                for interface_name in &obj.implements_interfaces {
                    match self.schema.type_definitions.get(interface_name) {
                        Some(def) => {
                            let ty = __Type::from_type_definition(self.schema, def);
                            interfaces.push(ty);
                        }
                        None => continue,
                    }
                }
                Some(interfaces)
            } else {
                None
            }
        } else {
            None
        }
    }

    async fn possible_types(&self) -> Option<Vec<__Type<'a>>> {
        if let TypeDetail::Named(def) = self.detail {
            match def {
                GqlTypeDefinition::Interface(interface) => {
                    let mut types = Vec::new();
                    for (_, ty) in &self.schema.type_definitions {
                        if let GqlTypeDefinition::Object(obj) = ty {
                            if obj.implements_interfaces.contains(&interface.name) {
                                let ty = __Type::from_type_definition(self.schema, ty);
                                types.push(ty);
                            }
                        }
                    }
                    Some(types)
                }
                GqlTypeDefinition::Union(uni) => {
                    let mut types = Vec::new();
                    for type_name in &uni.types {
                        if let Some(def) = self.schema.type_definitions.get(type_name) {
                            let ty = __Type::from_type_definition(self.schema, def);
                            types.push(ty);
                        }
                    }
                    Some(types)
                }
                _ => None,
            }
        } else {
            None
        }
    }

    async fn enum_values(&self) -> Option<Vec<__EnumValue>> {
        if let TypeDetail::Named(GqlTypeDefinition::Enum(enu)) = &self.detail {
            let mut values = Vec::new();
            for v in &enu.values {
                let value = __EnumValue::new(&v);
                values.push(value);
            }
            Some(values)
        } else {
            None
        }
    }

    async fn input_fields(&self) -> Option<Vec<__InputValue<'a>>> {
        if let TypeDetail::Named(GqlTypeDefinition::InputObject(input_obj)) = &self.detail {
            let mut values = Vec::new();
            for v in &input_obj.fields {
                let value = __InputValue::new(self.schema, &v);
                values.push(value);
            }
            Some(values)
        } else {
            None
        }
    }

    async fn of_type(&self) -> Option<__Type<'a>> {
        match self.detail {
            TypeDetail::Named(_) => None,
            TypeDetail::NonNull(type_name) => {
                let type_def = self.schema.type_definitions.get(type_name);
                match type_def {
                    Some(def) => Some(__Type::from_type_definition(self.schema, def)),
                    None => panic!("Unknown type: '{}'", type_name),
                }
            }
            TypeDetail::List(type_name) => {
                let type_def = self.schema.type_definitions.get(type_name);
                match type_def {
                    Some(def) => Some(__Type::from_type_definition(self.schema, def)),
                    None => panic!("Unknown type: '{}'", type_name),
                }
            }
        }
    }
}

#[async_trait::async_trait]
impl<'a> FieldResolver for __Type<'a> {
    async fn resolve_field(&self, ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        if ctx.item.name == "kind" {
            let kind = self.kind().await;
            let ctx_selection_set = ctx.with_selection_set(&ctx.item.selection_set);

            return SelectionSetResolver::resolve_selection_set(
                &kind.to_string(),
                &ctx_selection_set,
            )
            .await
            .map(Some);
        }

        if ctx.item.name == "name" {
            let name = self.name().await;
            let ctx_selection_set = ctx.with_selection_set(&ctx.item.selection_set);

            match name {
                Some(ty_name) => {
                    return SelectionSetResolver::resolve_selection_set(
                        ty_name,
                        &ctx_selection_set,
                    )
                    .await
                    .map(Some);
                }
                None => return Ok(None),
            }
        }

        if ctx.item.name == "description" {
            let desc = self.description().await;
            let ctx_selection_set = ctx.with_selection_set(&ctx.item.selection_set);

            match desc {
                Some(v) => {
                    return SelectionSetResolver::resolve_selection_set(v, &ctx_selection_set)
                        .await
                        .map(Some);
                }
                None => return Ok(None),
            }
        }

        if ctx.item.name == "fields" {
            let fields = self.fields().await;
            let ctx_selection_set = ctx.with_selection_set(&ctx.item.selection_set);

            match fields {
                Some(v) => {
                    return SelectionSetResolver::resolve_selection_set(&v, &ctx_selection_set)
                        .await
                        .map(Some);
                }
                None => return Ok(None),
            }
        }
        if ctx.item.name == "interfaces" {
            let interfaces = self.interfaces().await;
            let ctx_selection_set = ctx.with_selection_set(&ctx.item.selection_set);

            match interfaces {
                Some(v) => {
                    return SelectionSetResolver::resolve_selection_set(&v, &ctx_selection_set)
                        .await
                        .map(Some);
                }
                None => return Ok(None),
            }
        }
        if ctx.item.name == "possibleTypes" {
            let types = self.possible_types().await;
            let ctx_selection_set = ctx.with_selection_set(&ctx.item.selection_set);

            match types {
                Some(v) => {
                    return SelectionSetResolver::resolve_selection_set(&v, &ctx_selection_set)
                        .await
                        .map(Some);
                }
                None => return Ok(None),
            }
        }
        if ctx.item.name == "enumValues" {
            let values = self.enum_values().await;
            let ctx_selection_set = ctx.with_selection_set(&ctx.item.selection_set);

            match values {
                Some(v) => {
                    return SelectionSetResolver::resolve_selection_set(&v, &ctx_selection_set)
                        .await
                        .map(Some);
                }
                None => return Ok(None),
            }
        }
        if ctx.item.name == "inputFields" {
            let values = self.input_fields().await;
            let ctx_selection_set = ctx.with_selection_set(&ctx.item.selection_set);

            match values {
                Some(v) => {
                    return SelectionSetResolver::resolve_selection_set(&v, &ctx_selection_set)
                        .await
                        .map(Some);
                }
                None => return Ok(None),
            }
        }
        if ctx.item.name == "ofType" {
            let ty = self.of_type().await;
            let ctx_selection_set = ctx.with_selection_set(&ctx.item.selection_set);

            match ty {
                Some(v) => {
                    return SelectionSetResolver::resolve_selection_set(&v, &ctx_selection_set)
                        .await
                        .map(Some);
                }
                None => return Ok(None),
            }
        }
        Ok(None)
    }
    fn type_name() -> String {
        "__Type".to_string()
    }
}

#[async_trait::async_trait]
impl<'a> SelectionSetResolver for __Type<'a> {
    async fn resolve_selection_set(
        &self,
        ctx: &SelectionSetContext<'_>,
    ) -> ResolverResult<GqlValue> {
        ctx.resolve_selection_parallelly(self).await
    }
}
