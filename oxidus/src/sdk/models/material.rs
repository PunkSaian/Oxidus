use macros::vmt;


pub struct Material;

#[vmt]
pub struct Material {
    #[offset(12)]
    pub increment_refrence_count: extern "C" fn(),
}
