use proc_macro2::TokenStream;
use quote::{format_ident, quote_spanned};

use crate::parse::ParsedEnum;

impl ParsedEnum {
    /// Renders an implementation to resolve an enum from an `i32` and to convert it back to an
    /// `i32` if needed.
    ///
    /// These two below methods are accessible on the type as a result:
    ///
    /// - `_rid_from_discriminant` to resolve enum from `i32`
    /// - `_rid_into_discriminant` to convert the enum back to an `i32`
    ///
    /// NOTE: that only c-style enums are supported ATM as I'm not sure if we ever need to support
    /// rendering enums with arbitrary fields.
    ///
    /// The below example shows what impl is generated for a given enum.
    ///
    /// ```rust
    /// #[repr(C)]
    /// pub enum Filter {
    ///     All,
    ///     Completed,
    /// }
    ///
    /// impl Filter {
    ///     pub fn _rid_from_discriminant<T>(discriminant: T) -> Self
    ///     where
    ///         T: Into<i32> + Sized,
    ///     {
    ///         match discriminant.into() {
    ///             0 => Self::All,
    ///             1 => Self::Completed,
    ///             n => panic!("enum Filter does not include discriminant {}", n),
    ///         }
    ///     }
    ///
    ///     pub fn _rid_into_discriminant(&self) -> i32 {
    ///         match self {
    ///             Self::One => 0,
    ///             Self::Two => 1,
    ///         }
    ///     }
    /// }
    /// ```
    pub fn render_enum_resolution_impl(&self) -> TokenStream {
        let ident = &self.ident;
        let panic_quote = format!(
            "enum {ident} does not include discriminant {{}}",
            ident = ident
        );

        let variant_to_int_tokens: TokenStream = self
            .variants
            .iter()
            .map(|variant| {
                format!(
                    "{discriminant} => Self::{variant},\n",
                    discriminant = variant.discriminant,
                    variant = variant.ident,
                )
            })
            .collect::<Vec<String>>()
            .join("\n")
            .parse()
            .unwrap();

        let int_to_variant_tokens: TokenStream = self
            .variants
            .iter()
            .map(|variant| {
                format!(
                    "Self::{variant} => {discriminant},",
                    discriminant = variant.discriminant,
                    variant = variant.ident,
                )
            })
            .collect::<Vec<String>>()
            .join("\n")
            .parse()
            .unwrap();

        quote_spanned! { ident.span() =>
            impl #ident {
                pub fn _rid_from_discriminant<T>(discriminant: T) -> Self
                where
                    T: Into<i32> + Sized,
                {
                    match discriminant.into() {
                        #variant_to_int_tokens
                        n => panic!(#panic_quote, n),
                    }
                }
                pub fn _rid_into_discriminant(&self) -> i32 {
                    match self {
                        #int_to_variant_tokens
                    }
                }
            }
        }
    }
}
