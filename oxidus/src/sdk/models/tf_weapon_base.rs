use std::ffi::CStr;

use macros::vmt;
use super::super::bindings::TFWeaponBase;

#[vmt]
pub struct TFWeaponBase {
    #[offset(401)]
    pub _get_print_name: extern "c" fn() -> *const i8,
}

impl TFWeaponBase {
    pub fn get_print_name(&self) -> String {
        unsafe {
            let name = self._get_print_name();
            if name.is_null() {
                return "Unknown".to_owned();
            }
            CStr::from_ptr(name).to_string_lossy().into_owned()
        }
    }
}
