unsafe fn resolve_ptr_mut<'a, T>(ptr: *mut T) -> &'a mut T {
    assert!(!ptr.is_null());
    ptr.as_mut().unwrap()
}

pub fn resolve_ptr<'a, T>(ptr: *mut T) -> &'a T {
    unsafe { resolve_ptr_mut(ptr) }
}
