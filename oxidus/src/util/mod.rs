#[allow(deprecated)]
use std::ffi::CString;

use libc::{dlclose, dlopen, dlsym, RTLD_NOLOAD, RTLD_NOW};

pub mod consts;
pub mod error;
pub mod signature_scanner;

pub fn resolve_fn(module: &str, name: &str) -> Option<*mut ()> {
    unsafe {
        let module = CString::new(module).unwrap();
        let handle = dlopen(module.as_ptr(), RTLD_NOLOAD | RTLD_NOW);
        if handle.is_null() {
            return None;
        }
        let name = CString::new(name).unwrap();
        let res = dlsym(handle, name.as_ptr()).cast::<()>();
        dlclose(handle);
        if res.is_null() {
            return None;
        }
        Some(res)
    }
}

pub fn create_interface<T>(module_name: &str, interface_name: &str) -> Option<*mut T> {
    unsafe {
        let create_interface: extern "C" fn(*const i8, *const isize) -> *const () =
            std::mem::transmute(resolve_fn(module_name, "CreateInterface")?);
        Some(
            create_interface(
                CString::new(interface_name).unwrap().as_ptr(),
                std::ptr::null(),
            )
            .cast_mut()
            .cast::<T>(),
        )
    }
}
