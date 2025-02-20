use macros::vmt;

pub struct Client;

pub struct GuiSurface;

#[vmt]
pub struct GuiSurface {
    #[offset(51)]
    pub set_cursor: extern "C" fn(curosr: EMouseCursor),
    #[offset(52)]
    pub set_cursor_always_visible: extern "C" fn(visible: bool),
    #[offset(53)]
    pub is_cursor_visible: extern "C" fn() -> bool,
    #[offset(54)]
    pub apply_changes: extern "C" fn(),
    #[offset(61)]
    pub unlock_cursor: extern "C" fn(),
    #[offset(62)]
    pub lock_cursor: extern "C" fn(),
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
pub enum EMouseCursor {
    dc_user = 0,
    dc_none = 1,
    dc_arrow = 2,
    dc_ibeam = 3,
    dc_hourglass = 4,
    dc_waitarrow = 5,
    dc_crosshair = 6,
    dc_up = 7,
    dc_sizenw = 8,
    dc_sizese = 9,
    dc_sizene = 10,
    dc_sizesw = 11,
    dc_sizew = 12,
    dc_sizee = 13,
    dc_sizen = 14,
    dc_sizes = 15,
    dc_sizewe = 16,
    dc_sizens = 17,
    dc_sizeall = 18,
    dc_no = 19,
    dc_hand = 20,
    dc_blank = 21,
    dc_middle_pan = 22,
    dc_north_pan = 23,
    dc_north_east_pan = 24,
    dc_east_pan = 25,
    dc_south_east_pan = 26,
    dc_south_pan = 27,
    dc_south_west_pan = 28,
    dc_west_pan = 29,
    dc_north_west_pan = 30,
    dc_alias = 31,
    dc_cell = 32,
    dc_colresize = 33,
    dc_copycur = 34,
    dc_verticaltext = 35,
    dc_rowresize = 36,
    dc_zoomin = 37,
    dc_zoomout = 38,
    dc_help = 39,
    dc_custom = 40,
    dc_last = 41,
}
