use std::collections::HashMap;

use proc_macro2::TokenStream;
use syn::Ident;

use crate::{
    attrs::TypeInfoMap,
    common::{
        state::{get_state, ImplementationType},
        utils_module_tokens_if,
    },
    parse::{rust_type::RustType, ParsedFunction},
    render_common::{
        render_vec_accesses, PointerTypeAlias, RenderFunctionExportConfig,
        RenderableAccess, VecAccess,
    },
    render_dart::render_instance_method_extension,
    render_rust::{self, ffi_prelude, RenderedTypeAliasInfo},
};

use super::export_config::ExportConfig;

pub fn process_function_export(
    parsed_fn: &ParsedFunction,
    impl_ident: Option<Ident>,
    include_ffi: bool,
    ptr_type_aliases_map: &mut HashMap<String, TokenStream>,
    vec_accesses: &mut HashMap<String, VecAccess>,
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
        vec_access.map(|x| vec_accesses.insert(x.key(), x));

        tokens
    }
}

pub struct ExtractedTokens<'a> {
    pub vec_access_tokens: Vec<TokenStream>,
    pub ptr_typedef_tokens: Vec<&'a TokenStream>,
    pub utils_module: TokenStream,
}

pub fn extract_tokens<'a>(
    vec_accesses: HashMap<String, VecAccess>,
    ptr_type_aliases_map: &'a HashMap<String, TokenStream>,
    type_infos: &TypeInfoMap,
    config: &ExportConfig,
) -> ExtractedTokens<'a> {
    // -----------------
    // vec access
    // -----------------
    let vec_access_tokens = if config.render_vec_access {
        let needed_vec_accesses = get_state().need_implemtation(
            &ImplementationType::CollectionAccess,
            vec_accesses,
        );
        render_vec_accesses(&needed_vec_accesses, type_infos, "///")
    } else {
        vec![]
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
        vec_access_tokens,
        ptr_typedef_tokens,
        utils_module,
    }
}
