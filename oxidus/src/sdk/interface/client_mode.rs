use macros::vmt;

use crate::math::Angles;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ClientMode;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UserCmd {
    pub vmt: &'static (),
    pub command_number: i32,
    pub tick_count: i32,
    pub viewangles: Angles,
    pub forwardmove: f32,
    pub sidemove: f32,
    pub upmove: f32,
    pub buttons: Buttons,
    pub impulse: u8,
    pub weaponselect: i32,
    pub weaponsubtype: i32,
    pub seed: i32,
    pub mousedx: i16,
    pub mousedy: i16,
    pub hasbeenpredicted: bool,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Buttons(u32);

#[vmt]
pub struct ClientMode {
    #[offset(22)]
    pub create_move: extern "C" fn(input_sample_time: f32, cmd: &mut UserCmd) -> bool,
}
