use crate::{
    attrs::TypeInfoMap,
    parse::{ParsedStruct, ParsedStructField},
    render_dart::RenderDartTypeOpts,
};
use heck::MixedCase;
use quote::format_ident;
use syn::Ident;

impl ParsedStruct {
    /// Renders wrapper accessors for all fields on a given raw Pointer type, i.e. `RawStore`.
    /// Those wrappers are provided via an extension on the Dart type, i.e. `Store`.
    ///
    /// They take care of locking the store while looking up the value of those fields and convert
    /// them in one shot
    ///
    /// At this point the target of this extension is to be the `Store` since all accesses have to
    /// go through it in order to be safe. Were we to be able to pass a pointer to something that
    /// we got from the store it might be an invalid pointer by now and wouldn't be safe.
    pub fn render_field_wrappers(&self, comment: &str) -> Vec<String> {
        self.fields
            .iter()
            .map(|x| x.render_wrapper(&self.ident, self.type_infos(), comment))
            .collect()
    }
}

impl ParsedStructField {
    fn render_wrapper(
        &self,
        struct_ident: &Ident,
        type_infos: &TypeInfoMap,
        comment: &str,
    ) -> String {
        let rust_type_ident = self.rust_type.rust_ident();
        let field_ident =
            format_ident!("{}", self.ident.to_string().to_mixed_case());
        let raw_field_ident = &self.ident;
        let store_instance = struct_ident.to_string().to_mixed_case();

        let field_access = format!(
            "{ty}.{field}",
            ty = store_instance,
            field = raw_field_ident
        );

        let to_dart = self.dart_type.render_to_dart_for_snippet(&field_access);
        let dart_type = self
            .rust_type
            .render_dart_type(type_infos, RenderDartTypeOpts::attr());

        format!(
            r###"
{comment}     {Type} get {field} =>
{comment}       _read(({store}) => {to_dart}, '{store}.{raw_field}');"###,
            Type = dart_type,
            field = field_ident,
            raw_field = raw_field_ident,
            store = store_instance,
            to_dart = to_dart,
            comment = comment,
        )
    }
}
