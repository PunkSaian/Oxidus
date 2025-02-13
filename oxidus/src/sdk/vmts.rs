use macros::vmt;
use super::{bindings::*, collidable::Collidable};

#[vmt]
pub struct BaseEntity {
    #[offset(4)]
    pub get_collidable: extern "C" fn() -> &Collidable,
}

