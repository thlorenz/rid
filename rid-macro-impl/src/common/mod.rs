pub mod errors;
pub mod prefixes;
pub mod state;
mod syn_helpers;
pub mod tokens;
pub mod utils_module_tokens;

pub use errors::*;
pub use syn_helpers::*;
pub use utils_module_tokens::{utils_module_tokens, utils_module_tokens_if};

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
