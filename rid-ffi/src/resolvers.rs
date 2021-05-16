pub fn _option_ref_to_pointer<T>(
    option: ::std::option::Option<&T>,
) -> *const T {
    match option {
        Some(val) => val,
        None => ::std::ptr::null(),
    }
}
