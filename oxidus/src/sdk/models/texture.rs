
use macros::vmt;


pub struct Texture;

#[vmt]
pub struct Texture {
    #[offset(10)]
    pub increment_refrence_count: extern "C" fn(),
}
