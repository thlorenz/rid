use proc_macro2::Span;
use syn::spanned::Spanned;

type Tokens = proc_macro2::TokenStream;
pub fn type_error(ty: &syn::Type, err: &String) -> Tokens {
    let full_err = format!("rid: {}", err);
    syn::Error::new(ty.span(), full_err).to_compile_error()
}
pub fn derive_error(ident: &syn::Ident, err: &str) -> Tokens {
    let full_err = format!("rid: {}", err);
    syn::Error::new(ident.span(), full_err).to_compile_error()
}

pub fn callsite_error(err: &str) -> Tokens {
    let full_err = format!("rid: {}", err);
    syn::Error::new(Span::call_site(), full_err).to_compile_error()
}
