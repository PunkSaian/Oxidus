use macros::vmt;

#[repr(C)]
pub struct ConVar {
    _pad: [u8; 0x34],
    pub parent: &'static ConVar,
    pub default_value: *const i8,
    pub string: *const i8,
    pub string_length: i32,
    pub float_value: f32,
    pub int_value: i32,
    pub has_min: bool,
    pub min_val: f32,
    pub has_max: bool,
    pub max_val: f32,
    pub has_comp_min: bool,
    pub comp_min_val: f32,
    pub has_comp_max: bool,
    pub comp_max_val: f32,
    pub competitive_restrictions: bool,
    pub change_callback: *const (),
}

pub struct CVar {}

#[vmt]
pub struct CVar {
    #[offset(12)]
    pub find_var: extern "C" fn(name: *const i8) -> &'static ConVar,
    #[offset(13)]
    pub find_command: extern "C" fn(name: *const i8) -> &'static ConVar,
}

impl CVar {
    pub fn get_cvar(&self, name: &str) -> &'static ConVar {
        let name = std::ffi::CString::new(name).unwrap();
        self.find_var(name.as_ptr())
    }
}
