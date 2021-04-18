use syn::Ident;

use quote::format_ident;

pub fn fn_ident_and_impl_ident_string(
    fn_ident: &Ident,
    impl_ident: &Option<Ident>,
) -> (Ident, String) {
    let rid_impl_ident_str = match impl_ident {
        Some(ident) => format!("{}_", ident.to_string()),
        None => "".to_string(),
    };

    let rid_fn_ident =
        format_ident!("rid_export_{}{}", rid_impl_ident_str, fn_ident);

    (rid_fn_ident, rid_impl_ident_str)
}
