use syn::{FnArg, ImplItemMethod, Pat, PatIdent, Type, TypeReference};

pub fn get_method_args_without_context(
    method: &ImplItemMethod,
) -> Result<Vec<(PatIdent, Type)>, syn::Error> {
    let mut args = Vec::new();
    if method.sig.inputs.is_empty() {
        return Err(syn::Error::new_spanned(
            &method.sig,
            "self must be the first argument.",
        ));
    }

    for (index, arg) in method.sig.inputs.iter().enumerate() {
        if is_context_type(arg) {
            continue;
        }

        match arg {
            FnArg::Receiver(receiver) => {
                if index != 0 {
                    return Err(syn::Error::new_spanned(
                        receiver,
                        "self must be the first argument.",
                    ));
                }
            }

            FnArg::Typed(pat_type) => {
                if index == 0 {
                    return Err(syn::Error::new_spanned(
                        pat_type,
                        "self must be the first argument.",
                    ));
                }

                if let Pat::Ident(ident) = &*pat_type.pat {
                    args.push((ident.clone(), pat_type.ty.as_ref().clone()));
                } else {
                    return Err(syn::Error::new_spanned(pat_type, "Invalid arg"));
                }
            }
        }
    }
    Ok(args)
}

pub fn is_context_type(arg: &FnArg) -> bool {
    let mut is_context = false;
    if let FnArg::Typed(pat) = arg {
        if let Type::Reference(TypeReference { elem, .. }) = &*pat.ty {
            if let Type::Path(path) = elem.as_ref() {
                is_context = path.path.segments.last().unwrap().ident == "FieldContext";
            }
        }
    }
    is_context
}

pub fn is_result_type(return_type: &Type) -> bool {
    if let Type::Path(ty_path) = return_type {
        if ty_path.path.segments.last().unwrap().ident == "Result" {
            return true;
        }
    }
    false
}
