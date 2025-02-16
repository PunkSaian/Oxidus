use macros::vmt;

pub struct Client;

pub struct MatSystemSurface;

#[vmt]
pub struct MatSystemSurface {
    #[offset(61)]
    pub unlock_cursor: extern "C" fn(),
}
