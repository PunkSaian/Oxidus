use macros::vmt;

use super::{client_class::UnparsedClientClass, unknown::Unknown};

#[vmt]
pub struct Networkable {
    #[offset(0)]
    pub get_client_unknown: extern "C" fn() -> *const Unknown,
    #[offset(2)]
    pub get_client_class: extern "C" fn() -> *const UnparsedClientClass,
}

#[derive(Debug)]
pub struct Networkable;
