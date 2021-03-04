use super::parsed_variant::ParsedVariant;
use crate::common::{
    errors::derive_error,
    resolvers::{instance_ident, resolve_ptr},
    rust::RustType,
};
use quote::{format_ident, quote_spanned};
use rid_common::{DART_FFI, FFI_GEN_BIND, RID_FFI};
use syn::{punctuated::Punctuated, token::Comma, Variant};

type Tokens = proc_macro2::TokenStream;

pub struct ParsedEnum {
    pub ident: syn::Ident,
    pub parsed_variants: Vec<ParsedVariant>,
    pub method_prefix: String,
    module_ident: syn::Ident,
    ident_lower_camel: String,
}

impl ParsedEnum {
    pub fn new(ident: syn::Ident, variants: Punctuated<Variant, Comma>) -> Self {
        let ident_str = ident.to_string();
        let ident_lower_camel = lower_camel_case(&ident_str);
        let ident_lower = ident_str.to_lowercase();
        let method_prefix = format!("rid_{}", ident_lower);
        let module_ident = format_ident!("__rid_{}_ffi", ident_lower);
        let parsed_variants = parse_variants(variants, &method_prefix);
        Self {
            ident,
            parsed_variants,
            method_prefix,
            ident_lower_camel,
            module_ident,
        }
    }

    pub fn tokens(&self) -> Tokens {
        if self.parsed_variants.is_empty() {
            return Tokens::new();
        }
        let method_tokens = self.parsed_variants.iter().map(|v| self.rust_method(v));
        let dart_comment = self.dart_extension();
        let module_ident = &self.module_ident;

        quote_spanned! { self.ident.span() =>
            mod #module_ident {
              use super::*;
              #dart_comment
              #(#method_tokens)*
            }
        }
    }

    //
    // Rust Methods
    //

    fn rust_method(&self, variant: &ParsedVariant) -> Tokens {
        let variant_ident = &variant.ident;

        if variant.has_errors() {
            return variant
                .errors
                .iter()
                .map(|err| derive_error(variant_ident, err))
                .collect();
        }

        let fn_ident = &variant.method_ident;

        // TODO: how do we know what the model is?
        // If Msg is parsed first then we haven't even seen it yet.
        // Letting the user provide it as an attribute is easiest, but also makes him think that
        // there are options.
        // Possibly we could check the Model for update methods??? Complicated.
        let struct_ident = format_ident!("Model");
        let struct_instance_ident = instance_ident(&struct_ident);
        let resolve_struct_ptr = resolve_ptr(&struct_ident);

        let enum_ident = &self.ident;

        let arg_idents: Vec<(syn::Ident, syn::Ident)> = variant
            .fields
            .iter()
            .map(|f| {
                let ty = match &f.rust_ty {
                    RustType::Value(_) => todo!(),
                    RustType::Primitive(p) => p.to_string(),
                    RustType::Unknown => unimplemented!(),
                };

                (format_ident!("arg{}", f.slot), format_ident!("{}", ty))
            })
            .collect();

        let args = arg_idents
            .iter()
            .map(|(arg_name, ty)| quote_spanned! { fn_ident.span() => #arg_name: #ty });

        let msg_args = arg_idents
            .iter()
            .map(|(arg_name, _)| quote_spanned! { fn_ident.span() => #arg_name });

        quote_spanned! { variant_ident.span() =>
            #[no_mangle]
            #[allow(non_snake_case)]
            pub extern "C" fn #fn_ident(ptr: *mut #struct_ident, #(#args,)* ) {
                let mut #struct_instance_ident = #resolve_struct_ptr;
                let msg = #enum_ident::#variant_ident(#(#msg_args,)*);
                #struct_instance_ident.update(msg);
            }
        }
    }

    //
    // Dart Methods
    //
    fn dart_extension(&self) -> Tokens {
        let methods: Vec<String> = self
            .parsed_variants
            .iter()
            .map(|x| self.dart_method(x))
            .collect();

        let s = format!(
            r###"
/// The below extension provides convenience methods to send messages to rust.
///
/// ```dart
/// extension Rid_Message_ExtOnPointer{struct_ident}For{enum_ident} on {dart_ffi}.Pointer<{ffigen_bind}.{struct_ident}> {{
{methods}
/// }}
/// ```
        "###,
            enum_ident = self.ident,
            struct_ident = "Model", // TODO derive
            dart_ffi = DART_FFI,
            ffigen_bind = FFI_GEN_BIND,
            methods = methods.join("\n")
        );
        s.parse().unwrap()
    }

    fn dart_method(&self, variant: &ParsedVariant) -> String {
        let fn_ident = &variant.method_ident;

        let args_info: Vec<(usize, (String, String))> = variant
            .fields
            .iter()
            .map(|f| {
                (
                    format!("arg{}", f.slot),
                    format!("{}", f.dart_ty.to_string()),
                )
            })
            .enumerate()
            .collect();

        let args_decl = args_info
            .iter()
            .fold("".to_string(), |acc, (idx, (arg, ty))| {
                let comma = if *idx == 0 { "" } else { ", " };
                format!(
                    "{acc}{comma}{ty} {arg}",
                    acc = acc,
                    comma = comma,
                    ty = ty,
                    arg = arg
                )
            });

        let args_call = args_info
            .iter()
            .fold("".to_string(), |acc, (idx, (arg, _))| {
                let comma = if *idx == 0 { "" } else { ", " };
                format!("{acc}{comma}{arg}", acc = acc, comma = comma, arg = arg)
            });

        format!(
            "///   void {dart_method_name}({args_decl}) => {rid_ffi}.{method_name}(this, {args_call});",
            dart_method_name = self.dart_method_name(&fn_ident.to_string()),
            method_name = fn_ident.to_string(),
            args_decl = args_decl,
            args_call = args_call,
            rid_ffi = RID_FFI,
        )
    }

    fn dart_method_name(&self, rust_method_name: &str) -> String {
        // Cut off "rid_msg_
        let shortened = rust_method_name[8..].to_string();
        // lowercase first char
        format!("{}{}", self.ident_lower_camel, shortened)
    }
}

fn parse_variants(variants: Punctuated<Variant, Comma>, method_prefix: &str) -> Vec<ParsedVariant> {
    variants
        .into_iter()
        .map(|v| ParsedVariant::new(v, &method_prefix))
        .collect()
}

fn lower_camel_case(s: &str) -> String {
    format!("{}{}", s[0..1].to_lowercase(), s[1..].to_string())
}
