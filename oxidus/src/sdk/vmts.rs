use super::{bindings::BaseEntity, collidable::Collidable};
use macros::vmt;

#[vmt]
pub struct BaseEntity {
    #[offset(4)]
    pub get_collidable: extern "C" fn() -> &Collidable,
}
