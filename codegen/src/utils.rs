use syn::AttributeArgs;

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
