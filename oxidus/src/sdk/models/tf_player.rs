use crate::{
    i, prelude::Interfaces, sdk::{
        bindings::{BasePlayer, TFPlayer, TFWeaponBase},
        interface::engine::PlayerInfo,
    }
};
use macros::vmt;

#[vmt]
pub struct TFPlayer {
    #[offset(292)]
    pub _get_weapon: extern "c" fn() -> *const TFWeaponBase,
}

impl TFPlayer {
    pub fn get_info(&self) -> PlayerInfo {
        i!()
            .engine
            .get_player_info(self.get_entindex())
    }
    pub fn get_weapon(&self) -> Option<&TFWeaponBase> {
        let weapon = self._get_weapon();
        if weapon.is_null() {
            return None;
        }
        unsafe { Some(&*self._get_weapon()) }
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
