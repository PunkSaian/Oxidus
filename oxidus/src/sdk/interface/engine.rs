use macros::vmt;

pub struct Engine;

#[vmt]
pub struct Engine {
    #[offset(5)]
    pub get_screen_size: extern "C" fn(w: &mut isize, h: &mut isize)
}
