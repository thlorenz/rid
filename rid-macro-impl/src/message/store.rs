use proc_macro2::TokenStream;
use quote::quote_spanned;

use crate::common::state::{get_state, ImplementationType};
use rid_common::{
    DART_FFI, FFI_GEN_BIND, RID_DEBUG_LOCK, RID_DEBUG_REPLY, RID_FFI,
};
pub fn code_store_module(
    msg_ident: &syn::Ident,
    store_ident: &syn::Ident,
) -> TokenStream {
    let store_extension_dart: TokenStream = format!(
        r###"
/// ```dart
/// extension rid_store_specific_extension on {dart_ffi}.Pointer<{ffigen_bind}.{store}> {{
///   /// Executes the provided callback while locking the store to guarantee that the
///   /// store is not modified while that callback runs.
///   T runLocked<T>(T Function({dart_ffi}.Pointer<{ffigen_bind}.{store}>) fn) {{
///     try {{
///       ridStoreLock();
///       return fn(this);
///     }} finally {{
///       ridStoreUnlock();
///     }}
///   }}
///   /// Disposes the store and closes the Rust reply channel in order to allow the app
///   /// to exit properly. This needs to be called when exiting a Dart application.
///   Future<void> dispose() {{
///     return replyChannel.dispose();
///   }}
/// }}
/// ```
    "###,
        dart_ffi = DART_FFI,
        ffigen_bind = FFI_GEN_BIND,
        store = store_ident
    )
    .parse()
    .unwrap();

    let rid_store_lock_wrapper: TokenStream = format!(
        r###"
/// ```dart
/// int _locks = 0;
///
/// void Function(bool, int)? RID_DEBUG_LOCK = (bool locking, int locks) {{
///   String countIndicator = '';
///   if (locking) {{
///     for (int i = 1; i < locks; i++) countIndicator += '  ';
///     final lock = _locks == 1 ? '🔐' : '  ';
///     print('$lock $countIndicator{{');
///   }} else {{
///     for (int i = 0; i < locks; i++) countIndicator += '  ';
///     final lock = _locks == 0 ? '🔓' : '  ';
///     print('$countIndicator}} $lock');
///   }}
/// }};
///
/// void ridStoreLock() {{
///   if (_locks == 0) {rid_ffi}.rid_store_lock();
///   _locks++;
///   if (RID_DEBUG_LOCK != null) RID_DEBUG_LOCK!(true, _locks);
/// }}
///
/// void ridStoreUnlock() {{
///   _locks--;
///   if (RID_DEBUG_LOCK != null) RID_DEBUG_LOCK!(false, _locks);
///   if (_locks == 0) {rid_ffi}.rid_store_unlock();
/// }}
/// ```
"###,
        rid_ffi = RID_FFI,
    )
    .parse()
    .unwrap();

    let rid_create_store_wrapper: TokenStream = format!(
        r###"
/// ```dart
/// void _initRid() {{
///   print('Set {rid_debug_lock} to change if/how locking the rid store is logged');
///   print('Set {rid_debug_reply} to change if/how posted replies are logged');
/// }}
///
/// {dart_ffi}.Pointer<{ffigen_bind}.{store_struct}> createStore() {{
///   _initRid();
///   return {rid_ffi}.create_store();
/// }}
/// ```
"###, 
    store_struct = store_ident,
    rid_debug_lock = RID_DEBUG_LOCK,
    rid_debug_reply = RID_DEBUG_REPLY,
    rid_ffi = RID_FFI,
    ffigen_bind = FFI_GEN_BIND,
    dart_ffi = DART_FFI,
    )
    .parse()
    .unwrap();

    quote_spanned! {msg_ident.span() =>
        pub mod store {
            use super::*;

            /// cbindgen:ignore
            static mut STORE_LOCK: Option<::std::sync::RwLock<#store_ident>> = None;
            /// cbindgen:ignore
            static mut STORE_ACCESS: Option<RidStoreAccess> = None;
            /// cbindgen:ignore
            static INIT_STORE: ::std::sync::Once = ::std::sync::Once::new();
            /// cbindgen:ignore
            static mut LOCK_READ_GUARD: Option<
                ::std::sync::RwLockReadGuard<'static, #store_ident>,
            > = None;

            struct RidStoreAccess {
                lock: &'static ::std::sync::RwLock<#store_ident>,
            }

            impl RidStoreAccess {
                fn instance() -> &'static RidStoreAccess {
                    unsafe {
                        INIT_STORE.call_once(|| {
                            STORE_LOCK = Some(::std::sync::RwLock::new(
                                #store_ident::create(),
                            ));
                            STORE_ACCESS = Some(RidStoreAccess {
                                lock: STORE_LOCK.as_ref().unwrap(),
                            });
                        });
                        STORE_ACCESS.as_ref().unwrap()
                    }
                }
            }

            // -----------------
            // API used by rid internally and for multi threading scenarios
            // -----------------

            /// Locks store for reading and allows non-mutable access
            /// A read lock can be aquired when no other write lock is in use.
            /// Multiple read locks can be given out in parallel.
            pub fn read() -> ::std::sync::RwLockReadGuard<'static, #store_ident> {
                RidStoreAccess::instance().lock.read().unwrap()
            }

            /// Locks store for writing and allows mutable access
            /// A write lock can be aquired when no other read nor write lock is in use.
            /// Only one write lock can be aquired.
            pub fn write() -> ::std::sync::RwLockWriteGuard<'static, #store_ident> {
                RidStoreAccess::instance().lock.write().unwrap()
            }

            // -----------------
            // Dart Access to create and lock/unlock store
            // -----------------
            #rid_create_store_wrapper
            #[no_mangle]
            pub extern "C" fn create_store() -> *const #store_ident {
                let store = RidStoreAccess::instance().lock.read().unwrap();
                &*store as *const #store_ident
            }

            #rid_store_lock_wrapper
            #[no_mangle]
            pub extern "C" fn rid_store_lock() {
                if unsafe { LOCK_READ_GUARD.is_some() } {
                    eprintln!("WARN trying to lock an already locked store");
                } else {
                    unsafe {
                        LOCK_READ_GUARD = Some(read());
                    }
                }
            }

            #[no_mangle]
            pub extern "C" fn rid_store_unlock() {
                if unsafe { LOCK_READ_GUARD.is_none() } {
                    eprintln!("WARN trying to unlock an already unlocked store");
                } else {
                    unsafe {
                        LOCK_READ_GUARD = None;
                    }
                }
            }

            #store_extension_dart
            #[no_mangle]
            pub extern "C" fn rid_store_free() {
                // We may want to figure out a way to drop the store here in the future, even
                // though that isn't necessary as the app will exit after the store was freed.
                // For now we just make sure we wait for any thread that as a read or write lock
                // to complete before we return from this method.
                let _write_lock = write();
            }
        }
    }
}
