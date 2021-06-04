pub mod dart;
pub mod errors;
pub mod parsed_field;
pub mod prefixes;
pub mod rust;
pub mod state;
mod syn_helpers;
pub mod tokens;

pub use dart::DartType;
pub use errors::*;
pub use parsed_field::ParsedField;
pub use rust::{extract_path_segment, PrimitiveType, RustType, ValueType};
pub use syn_helpers::*;

// Test replacements
#[cfg(not(test))]
pub use proc_macro_error::abort;

#[cfg(test)]
#[macro_export]
macro_rules! abort {
    ($err:expr) => {
        panic!($err)
    };
    ($span:expr, $tts:tt) => {
        panic!("proc_macro_error::abort! for:\n\n{:?}\n\n{}", $span, $tts)
    };
    ($span:expr, $($tts:tt)*) => {
        panic!("proc_macro_error::abort! for:\n\n{:?}\n\n{}", $span, format!($($tts)*))
    };
}
#[cfg(test)]
pub use abort;

#[cfg(test)]
mod test;
#[cfg(test)]
pub use test::*;
