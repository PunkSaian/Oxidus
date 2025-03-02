use std::ffi::{CStr, CString};

use macros::vmt;

use crate::sdk::models::{material::Material, texture::Texture};

use super::material_render_context::MaterialRenderContext;

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum CreateRenderTargetFlags {
    HDR = 0x1,
    AUTOMIPMAP = 0x2,
    UNFILTERABLE_OK = 0x4,
    NOEDRAM = 0x8,
    TEMP = 0x10,
}

#[repr(C)]
pub enum SizeMode {
    NoChange = 0,
    DEFAULT = 1,
    PICMIP = 2,
    HDR = 3,
    FullFrameBuffer = 4,
    OFFSCREEN = 5,
    FullFrameBufferRoundedUp = 6,
    ReplayScreenshot = 7,
    LITERAL = 8,
    LiteralPicmip = 9,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum ImageFormat {
    UNKNOWN = -1,
    RGBA8888 = 0,
    ABGR8888,
    RGB888,
    BGR888,
    RGB565,
    I8,
    IA88,
    P8,
    A8,
    RGB888_BLUESCREEN,
    BGR888_BLUESCREEN,
    ARGB8888,
    BGRA8888,
    DXT1,
    DXT3,
    DXT5,
    BGRX8888,
    BGR565,
    BGRX5551,
    BGRA4444,
    DXT1_ONEBITALPHA,
    BGRA5551,
    UV88,
    UVWQ8888,
    RGBA16161616F,
    RGBA16161616,
    UVLX8888,
    R32F,
    RGB323232F,
    RGBA32323232F,
    NV_DST16,
    NV_DST24,
    NV_INTZ,
    NV_RAWZ,
    ATI_DST16,
    ATI_DST24,
    NV_NULL,
    ATI2N,
    ATI1N,
    DXT1_RUNTIME,
    DXT5_RUNTIME,
}

#[repr(C)]
pub enum TargetDepth {
    SHARED,
    SEPARATE,
    NONE,
    ONLY,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum CompiledVtfFlags {
    POINTSAMPLE = 0x1,
    TRILINEAR = 0x2,
    CLAMPS = 0x4,
    CLAMPT = 0x8,
    ANISOTROPIC = 0x10,
    HINT_DXT5 = 0x20,
    SRGB = 0x40,
    NORMAL = 0x80,
    NOMIP = 0x100,
    NOLOD = 0x200,
    ALL_MIPS = 0x400,
    PROCEDURAL = 0x800,
    ONEBITALPHA = 0x1000,
    EIGHTBITALPHA = 0x2000,
    ENVMAP = 0x4000,
    RENDERTARGET = 0x8000,
    DEPTHRENDERTARGET = 0x10000,
    NODEBUGOVERRIDE = 0x20000,
    SINGLECOPY = 0x40000,
    STAGING_MEMORY = 0x80000,
    IMMEDIATE_CLEANUP = 0x10_0000,
    IGNORE_PICMIP = 0x0020_0000,
    UNUSED_00400000 = 0x0040_0000,
    NODEPTHBUFFER = 0x0080_0000,
    UNUSED_01000000 = 0x0100_0000,
    CLAMPU = 0x0200_0000,
    VERTEXTEXTURE = 0x0400_0000,
    SSBUMP = 0x0800_0000,
    Unused10000000 = 0x1000_0000,
    BORDER = 0x2000_0000,
    STREAMABLE_COARSE = 0x4000_0000,
    STREAMABLE_FINE = 0x8000_0000,
}

pub struct MaterialSystem;

#[vmt]
pub struct MaterialSystem {
    #[offset(79)]
    pub find_material: extern "C" fn(
        name: *const i8,
        texture_group: *const i8,
        complain: bool,
        complain_prefix: *const i8,
    ) -> &'static mut Material,
    #[offset(85)]
    pub create_render_target_texture_ex: extern "C" fn(
        name: CString,
        w: i32,
        h: i32,
        size_mode: SizeMode,
        format: ImageFormat,
        depth: TargetDepth,
        texture_flags: i64,
        render_target_flags: CreateRenderTargetFlags,
    ) -> &'static mut Texture,
    #[offset(98)]
    pub getn_render_context: extern "C" fn() -> &'static mut MaterialRenderContext,
}
