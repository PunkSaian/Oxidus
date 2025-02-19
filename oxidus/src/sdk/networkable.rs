use macros::vmt;

use super::{client_class::{ClientClass, UnparsedClientClass}, unknown::Unknown};


#[derive(Debug)]
pub struct Networkable;

#[vmt]
pub struct Networkable {
    #[offset(0)]
    pub get_client_unknown: extern "C" fn() -> *const Unknown,
    #[offset(2)]
    pub _get_client_class: extern "C" fn() -> *const UnparsedClientClass,
    #[offset(9)]
    pub get_index: extern "C" fn() -> i32,
}

impl Networkable {

    pub fn get_client_class(&self) -> ClientClass {
        unsafe { &*self._get_client_class() }.parse()
    }

}
