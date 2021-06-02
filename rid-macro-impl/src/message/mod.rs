mod attach;
pub mod parsed_enum;
pub mod parsed_variant;
mod store;
pub mod variant_field;

pub use attach::rid_ffi_message_impl;
pub use variant_field::*;
