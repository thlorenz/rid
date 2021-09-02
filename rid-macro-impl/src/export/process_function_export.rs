use std::collections::HashMap;

use proc_macro2::TokenStream;
use syn::Ident;

use crate::{
    accesses::{render_collection_accesses, RenderableAccess},
    attrs::TypeInfoMap,
    common::{
        state::{get_state, ImplementationType},
        utils_module_tokens_if,
    },
    parse::{rust_type::RustType, ParsedFunction},
    render_common::{PointerTypeAlias, RenderFunctionExportConfig},
    render_dart::render_instance_method_extension,
    render_rust::{self, ffi_prelude, RenderedTypeAliasInfo},
};

use super::export_config::ExportConfig;

pub fn process_function_export(
    parsed_fn: &ParsedFunction,
    impl_ident: Option<Ident>,
    include_ffi: bool,
    ptr_type_aliases_map: &mut HashMap<String, TokenStream>,
    accesses: &mut HashMap<String, Box<dyn RenderableAccess>>,
) -> TokenStream {
    {
        let render_rust::RenderedFunctionExport {
            tokens,
            ptr_type_aliases,
            vec_access,
        } = render_rust::render_function_export(
            parsed_fn,
            impl_ident,
            Some(RenderFunctionExportConfig {
                include_ffi,
                ..Default::default()
            }),
        );
        for PointerTypeAlias {
            alias,
            typedef,
            type_name,
            needs_free,
        } in ptr_type_aliases
        {
            ptr_type_aliases_map.insert(alias.to_string(), typedef.clone());
            if needs_free {
                // NOTE: if this ever throws we need to undo part of the change which removed free.
                // It will most likely be needed again when we return owned custom structs from an
                // export as things like RidVec render their own free methods.
                // We should reintroduce it in a manner that doesn't need the impl owner type
                // to be provided and possibly render the free call in place using state to avoid
                // rendering duplicates.
                panic!("Rid removed free implementation for Rust and Dart since it wasn't complete nor used");
            }
        }
        vec_access.map(|x| accesses.insert(x.key(), Box::new(x)));

        tokens
    }
}

pub struct ExtractedTokens<'a> {
    pub access_tokens: TokenStream,
    pub ptr_typedef_tokens: Vec<&'a TokenStream>,
    pub utils_module: TokenStream,
}

pub fn extract_tokens<'a>(
    accesses: HashMap<String, Box<dyn RenderableAccess>>,
    ptr_type_aliases_map: &'a HashMap<String, TokenStream>,
    type_infos: &TypeInfoMap,
    config: &ExportConfig,
) -> ExtractedTokens<'a> {
    // -----------------
    // Collection Accesses
    // -----------------
    let access_tokens = if config.render_collection_access {
        render_collection_accesses(
            accesses,
            type_infos,
            &Default::default(),
            &Default::default(),
        )
        .0
    } else {
        TokenStream::new()
    };

    // -----------------
    // Type Aliases
    // -----------------
    let ptr_typedef_tokens: Vec<&TokenStream> =
        ptr_type_aliases_map.values().collect();

    // -----------------
    // Utils Module (guarded to render only once)
    // -----------------
    let utils_module = utils_module_tokens_if(config.render_utils_module);

    ExtractedTokens {
        access_tokens,
        ptr_typedef_tokens,
        utils_module,
    }
}
