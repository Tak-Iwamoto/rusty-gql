use proc_macro2::TokenStream;

pub enum ProcMacroError {
    Syn(syn::Error),
    Darling(darling::Error),
}

impl ProcMacroError {
    pub fn write_errors(self) -> TokenStream {
        match self {
            ProcMacroError::Syn(err) => err.to_compile_error(),
            ProcMacroError::Darling(err) => err.write_errors(),
        }
    }
}
