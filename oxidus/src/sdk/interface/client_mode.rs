use std::{mem::transmute, ptr};

use macros::vmt;

use crate::math::Angles;

use super::client::ViewSetup;

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

#[repr(C)]
#[derive(Debug, Clone)]
pub enum ButtonFlags {
    InAttack,
    InJump,
    InDuck,
    InForward,
    InBack,
    InUse,
    InCancel,
    InLeft,
    InRight,
    InMoveleft,
    InMoveright,
    InAttack2,
    InRun,
    InReload,
    InAlt1,
    InAlt2,
    InScore,
    InSpeed,
    InWalk,
    InZoom,
    InWeapon1,
    InWeapon2,
    InBullrush,
    InGrenade1,
    InGrenade2,
    InAttack3,
}

impl Buttons {
    pub fn get(self, flag: ButtonFlags) -> bool {
        let flag = flag as u8;
        let b: u32 = unsafe { transmute::<Self, u32>(self) };
        (b & (1 << flag)) != 0
    }
    pub fn set(&mut self, flag: ButtonFlags, val: bool) {
        let flag = flag as u8;
        let b: &mut u32 = unsafe { &mut *ptr::from_mut::<Buttons>(self).cast::<u32>() };
        if val {
            *b |= 1 << flag;
        } else {
            *b &= !(1 << flag);
        }
    }
}

#[vmt]
pub struct ClientMode {
    #[offset(17)]
    pub override_view: extern "C" fn(view_setup: &mut ViewSetup) -> bool,
    #[offset(22)]
    pub create_move: extern "C" fn(input_sample_time: f32, cmd: &mut UserCmd) -> bool,
}
