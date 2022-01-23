pub struct possibleTypes;

impl CustomDirective for possibleTypes {
    fn resolve_field(&self, ctx: &FieldContext<'_>, directive_args: &BTreeMap<String, GqlValue>, resolve_fut: ResolveFut<'_>) -> ResolverResult<Option<GqlValue>> {
        todo!()
    }
}