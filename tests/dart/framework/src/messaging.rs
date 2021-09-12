#[rid::export]
pub fn send_log_warn_message(id: u8) -> u8 {
    rid::log_warn!("Warn {} from Rust", id);
    0
}

#[rid::export]
pub fn send_log_info_message(id: u8) -> u8 {
    rid::log_info!("Info {} from Rust", id);
    0
}

#[rid::export]
pub fn send_log_debug_message(id: u8) -> u8 {
    rid::log_debug!("Debug {} from Rust", id);
    0
}

#[rid::export]
pub fn send_error_message_with_details(id: u8) -> u8 {
    rid::error!(format!("Error {} from Rust", id), "Some Error Details");
    0
}

#[rid::export]
pub fn send_severe_error_message_with_details(id: u8) -> u8 {
    rid::severe!(
        &format!("Severe Error {} from Rust", id),
        "Some Severe Error Details"
    );
    0
}

#[rid::export]
pub fn send_error_message_without_details(id: u8) -> u8 {
    rid::error!(format!("Error {} from Rust", id));
    0
}

#[rid::export]
pub fn send_severe_error_message_without_details(id: u8) -> u8 {
    rid::severe!(&format!("Severe Error {} from Rust", id));
    0
}
