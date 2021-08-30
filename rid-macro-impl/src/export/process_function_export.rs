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
    frees: &mut HashMap<String, PointerTypeAlias>,
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
                frees.insert(
                    type_name.clone(),
                    PointerTypeAlias {
                        alias,
                        typedef,
                        type_name,
                        needs_free,
                    },
                );
            }
        }
        vec_access.map(|x| vec_accesses.insert(x.key(), x));

        tokens
    }
}

pub struct ExtractedTokens<'a> {
    pub vec_access_tokens: Vec<TokenStream>,
    pub free_tokens: Vec<TokenStream>,
    pub dart_free_extensions_tokens: Vec<TokenStream>,
    pub ptr_typedef_tokens: Vec<&'a TokenStream>,
    pub utils_module: TokenStream,
}

pub fn extract_tokens<'a>(
    vec_accesses: HashMap<String, VecAccess>,
    frees: HashMap<String, PointerTypeAlias>,
    ptr_type_aliases_map: &'a HashMap<String, TokenStream>,
    parsed_impl_ty: &RustType,
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
    // frees
    // -----------------
    let free_tokens = if config.render_frees {
        let needed_frees = get_state()
            .need_implemtation(&ImplementationType::Free, frees.clone());
        needed_frees
            .into_iter()
            .map(|x| x.render_free(ffi_prelude()))
            .collect()
    } else {
        vec![]
    };

    let (infos, free_tokens) = unpack_tuples(free_tokens);

    // TODO: non-instance method strings
    let dart_free_extensions_tokens =
        if config.render_frees && config.render_dart_free_extension {
            infos
                .into_iter()
                .map(|RenderedTypeAliasInfo { alias, fn_ident }| {
                    parsed_impl_ty
                        .render_dart_dispose_extension(fn_ident, &alias, "///")
                })
                .collect()
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
        free_tokens,
        dart_free_extensions_tokens,
        ptr_typedef_tokens,
        utils_module,
    }
}

// -----------------
// Utils
// -----------------
fn unpack_tuples<T, U>(tpls: Vec<(T, U)>) -> (Vec<T>, Vec<U>) {
    let mut xs: Vec<T> = Vec::with_capacity(tpls.len());
    let mut ys: Vec<U> = Vec::with_capacity(tpls.len());
    for (x, y) in tpls {
        xs.push(x);
        ys.push(y);
    }

    (xs, ys)
}
