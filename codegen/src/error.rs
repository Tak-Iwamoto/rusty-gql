use proc_macro2::TokenStream;

pub enum CodegenError {
    Darling(darling::Error),
    Syn(syn::Error),
}

impl CodegenError {
    pub fn to_token_stream(&self) -> TokenStream {
        match self {
            // CodegenError::Darling(darling_err) => *darling_err.write_errors(),
            CodegenError::Darling(darling_err) => unreachable!(),
            CodegenError::Syn(syn_err) => syn_err.to_compile_error(),
        }
    }
}

pub type CodegenResult<T> = std::result::Result<T, CodegenError>;
