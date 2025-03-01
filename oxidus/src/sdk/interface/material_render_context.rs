use macros::vmt;

use crate::sdk::models::texture::Texture;

pub struct MaterialRenderContext;

#[repr(C)]
pub enum MaterialCullMode {
    Ccw,
    Cw,
}
#[repr(C)]
pub enum StencilComparisonFn {
    Never = 1,
    Less = 2,
    Equal = 3,
    LlesEqual = 4,
    Greater = 5,
    NotEqual = 6,
    GreaterEqual = 7,
    Alaway = 8,
    ForceDword = 0x7fff_ffff,
}

#[repr(C)]
pub enum StencilOperation {
    Keep = 1,
    Zero = 2,
    Replace = 3,
    IncrSat = 4,
    DecrSat = 5,
    Invert = 6,
    Incr = 7,
    Decr = 8,
    ForceDword = 0x7fff_ffff,
}

#[vmt]
pub struct MaterialRenderContext {
    #[offset(6)]
    pub set_render_target: extern "C" fn(texture: &mut Texture),
    #[offset(10)]
    pub viewport: extern "C" fn(x: i32, y: i32, width: i32, height: i32),
    #[offset(12)]
    pub clear_buffers: extern "C" fn(clear_color: bool, clear_depth: bool, clear_stencil: bool),
    #[offset(38)]
    pub cull_mode: extern "C" fn(cull_mode: MaterialCullMode),
    #[offset(73)]
    pub clear_color_4ub: extern "C" fn(r: u8, g: u8, b: u8, a: u8),
    #[offset(105)]
    pub push_render_target_and_viewport: extern "C" fn(),
    // STENCIL
    #[offset(117)]
    pub set_stencil_enable: extern "C" fn(enable: bool),
    #[offset(118)]
    pub set_stencil_fail_operation: extern "C" fn(operation: StencilOperation),
    #[offset(119)]
    pub set_stencil_zfail_operation: extern "C" fn(operation: StencilOperation),
    #[offset(120)]
    pub set_stencil_pass_operation: extern "C" fn(operation: StencilOperation),
    #[offset(121)]
    pub set_stencil_compare_function: extern "C" fn(compare_fn: StencilComparisonFn),
    #[offset(122)]
    pub set_stencil_refrence_value: extern "C" fn(val: i32),
    #[offset(123)]
    pub set_stencil_test_mask: extern "C" fn(mask: u32),
    #[offset(124)]
    pub set_stencil_write_mask: extern "C" fn(mask: u32),
}
