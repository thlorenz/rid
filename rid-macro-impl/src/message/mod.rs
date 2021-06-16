mod attach;
pub mod parsed_message_enum;
pub mod parsed_variant;
pub mod render_message_enum;
mod store;
pub mod variant_field;

pub use attach::rid_message_impl;
pub use parsed_message_enum::*;
pub use variant_field::*;

#[cfg(test)]
pub mod message_test;
