use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use super::render_swift_call;

pub fn render_swift_display_call(
    fn_display_method: &str,
    comment: &str,
) -> String {
    format!(
        r###"
{comment} Swift call generated for exported display method of '{method_ident}'.
{comment} 
{comment} ```swift
{comment} func dummyCall_{method_ident}() {{
{comment}     {call}
{comment} }}
{comment} ```
"###,
        comment = comment,
        method_ident = fn_display_method,
        call = render_swift_call(fn_display_method, &[], true)
    )
}
