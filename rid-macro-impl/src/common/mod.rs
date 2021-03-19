pub mod dart;
pub mod errors;
pub mod parsed_field;
pub mod parsed_receiver;
pub mod parsed_reference;
pub mod resolvers;
pub mod rust;
pub mod rust_arg;
pub mod state;

pub use dart::DartType;
pub use errors::*;
pub use parsed_field::ParsedField;
pub use parsed_receiver::*;
pub use parsed_reference::*;
pub use rust::{extract_path_segment, PrimitiveType, RustType, ValueType};
pub use rust_arg::*;

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
