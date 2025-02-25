use std::{fmt::Display, mem::transmute};

use crate::{
    i,
    sdk::{
        bindings::{BasePlayer, TFPlayer, TFWeaponBase},
        interface::engine::PlayerInfo,
    },
};
use macros::vmt;

#[derive(Debug, Clone, Copy)]
pub struct Flags(u32);
impl Flags {
    pub fn get(self, flag: Flag) -> bool {
        let flag = flag as u8;
        let shifted = 1 << flag;
        let Flags(b) = self;
        b & shifted == shifted
    }
}
impl Display for Flags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let flags = (1..Flag::Transragdoll as u32)
            .filter_map(|flag| {
                let flag: Flag = unsafe { transmute(flag) };
                if self.get(flag) {
                    return Some(format!("{flag:?}"));
                }
                None
            })
            .collect::<Vec<_>>();
        write!(f, "{}", flags.join(" | "))
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum Flag {
    Onground,
    Ducking,
    Waterjump,
    Ontrain,
    Inrain,
    Frozen,
    Atcontrols,
    Client,
    Fakeclient,
    Inwater,
    Fly,
    Swim,
    Conveyor,
    Npc,
    Godmode,
    Notarget,
    Aimtarget,
    Partialground,
    Staticprop,
    Graphed,
    Grenade,
    Stepmovement,
    Donttouch,
    Basevelocity,
    Worldbrush,
    Object,
    Killme,
    Onfire,
    Dissolving,
    Transragdoll,
    UnblockableByPlayer,
}

#[vmt]
pub struct TFPlayer {
    #[offset(292)]
    pub _get_weapon: extern "c" fn() -> *const TFWeaponBase,
}

impl TFPlayer {
    pub fn get_info(&self) -> PlayerInfo {
        i!().engine.get_player_info(self.get_entindex())
    }
    pub fn get_weapon(&self) -> Option<&TFWeaponBase> {
        let weapon = self._get_weapon();
        if weapon.is_null() {
            return None;
        }
        unsafe { Some(&*self._get_weapon()) }
    }
    pub fn get_flags(&mut self) -> &mut Flags {
        unsafe { transmute::<&mut i32, &mut Flags>(&mut self.m_fFlags) }
    }
}

impl<'a> From<&'a TFPlayer> for &'a BasePlayer {
    fn from(val: &'a TFPlayer) -> Self {
        unsafe { &*std::ptr::from_ref::<TFPlayer>(val).cast::<BasePlayer>() }
    }
}

impl<'a> From<&'a mut TFPlayer> for &'a mut BasePlayer {
    fn from(val: &'a mut TFPlayer) -> Self {
        unsafe { &mut *std::ptr::from_mut::<TFPlayer>(val).cast::<BasePlayer>() }
    }
}
