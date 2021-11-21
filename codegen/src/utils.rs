use syn::{AttributeArgs, Ident, ImplItemMethod, Pat, PatIdent, ReturnType, Type, TypeParamBound};

fn check_path_name(path: &syn::Path, value: &str) -> bool {
    path.segments.len() == 1 && path.segments[0].ident == value
}

pub fn find_attr(args: &AttributeArgs, name: &str) -> Option<String> {
    for arg in args {
        if let syn::NestedMeta::Meta(syn::Meta::NameValue(name_value)) = arg {
            if name_value.path.is_ident(name) {
                if let syn::Lit::Str(lit) = &name_value.lit {
                    return Some(lit.value());
                }
            }
        } else {
            continue;
        }
    }
    None
}

pub fn get_type_name(ty: &Type) -> Result<String, syn::Error> {
    match ty {
        Type::Group(group) => get_type_name(&group.elem),
        Type::TraitObject(trait_object) => Ok(trait_object
            .bounds
            .iter()
            .find_map(|bound| match bound {
                TypeParamBound::Trait(t) => {
                    Some(t.path.segments.last().map(|s| s.ident.to_string()).unwrap())
                }
                _ => None,
            })
            .unwrap()),
        Type::Path(path) => Ok(path
            .path
            .segments
            .last()
            .map(|segment| segment.ident.to_string())
            .unwrap()),
        _ => Err(syn::Error::new_spanned(ty, "invalid type")),
    }
}

pub fn get_method_args(method: &ImplItemMethod) -> Result<Vec<proc_macro2::Ident>, syn::Error> {
    let mut args = Vec::new();
    if method.sig.inputs.is_empty() {}

    for (index, arg) in method.sig.inputs.iter().enumerate() {
        match arg {
            syn::FnArg::Receiver(receiver) => {
                if index != 0 {
                    return Err(syn::Error::new_spanned(
                        receiver,
                        "self must be the first argument.",
                    ));
                }
            }
            syn::FnArg::Typed(pat_type) => {
                if index == 0 {
                    return Err(syn::Error::new_spanned(
                        pat_type,
                        "self must be the first argument.",
                    ));
                }

                if let Pat::Ident(ident) = &*pat_type.pat {
                    args.push(ident.ident.clone());
                } else {
                    return Err(syn::Error::new_spanned(pat_type, "Invalid arg"));
                }
            }
        }
    }
    Ok(args)
}

pub fn is_result_type(return_type: &Type) -> bool {
    if let Type::Path(ty_path) = return_type {
        if ty_path.path.segments.last().unwrap().ident == "Result" {
            return true;
        }
    }
    false
}
