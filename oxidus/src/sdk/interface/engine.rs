use std::mem::MaybeUninit;

use imgui::sys::cty::c_char;
use macros::vmt;

use crate::{i, sdk::bindings::{BaseEntity, TFPlayer}};

pub struct Engine;

const MAX_PLAYER_NAME_LENGTH: usize = 32;
const SIGNED_GUID_LEN: usize = 32;
const MAX_CUSTOM_FILES: usize = 4;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct PlayerInfoUnparsed {
    pub name: [u8; MAX_PLAYER_NAME_LENGTH],
    pub user_id: i32,
    pub guid: [u8; SIGNED_GUID_LEN + 1],
    pub friends_id: u32,
    pub friends_name: [u8; MAX_PLAYER_NAME_LENGTH],
    pub fakeplayer: bool,
    pub ishltv: bool,
    pub custom_files: [u32; MAX_CUSTOM_FILES],
    pub files_downloaded: c_char,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct PlayerInfo {
    pub name: String,
    pub user_id: i32,
    pub guid: String,
    pub friends_id: u32,
    pub friends_name: String,
    pub fakeplayer: bool,
    pub ishltv: bool,
    pub custom_files: [u32; MAX_CUSTOM_FILES],
    pub files_downloaded: c_char,
}

impl From<PlayerInfoUnparsed> for PlayerInfo {
    fn from(value: PlayerInfoUnparsed) -> Self {
        let str_from_arr = |arr: Vec<u8>| -> String {
            unsafe {
                String::from_utf8_unchecked(arr)
                    .chars()
                    .filter(|char| *char != '\0')
                    .collect()
            }
        };
        PlayerInfo {
            name: str_from_arr(value.name.to_vec()),
            user_id: value.user_id,
            guid: str_from_arr(value.guid.to_vec()),
            friends_id: value.friends_id,
            friends_name: str_from_arr(value.friends_name.to_vec()),
            fakeplayer: value.fakeplayer,
            ishltv: value.ishltv,
            custom_files: value.custom_files,
            files_downloaded: value.files_downloaded,
        }
    }
}

//INFO: FIXED
#[vmt]
pub struct Engine {
    #[offset(5)]
    pub get_screen_size: extern "C" fn(w: &mut isize, h: &mut isize),
    #[offset(8)]
    pub _get_player_info: extern "C" fn(index: i32, info: &mut PlayerInfoUnparsed),
    #[offset(12)]
    pub get_local_player_entindex: extern "C" fn() -> i32,
    #[offset(26)]
    pub is_in_game: extern "C" fn() -> bool,
}

impl Engine {
    pub fn get_player_info(&self, index: i32) -> PlayerInfo {
        unsafe {
            let mut info = MaybeUninit::zeroed().assume_init();
            self._get_player_info(index, &mut info);
            info.into()
        }
    }
    pub fn get_local_player(&self) -> Option<&'static mut TFPlayer> {
        let ent = i!() 
            .entity_list
            .get_client_entity_from_index(self.get_local_player_entindex())?;

        Some(unsafe { &mut *std::ptr::from_mut::<BaseEntity>(ent).cast::<TFPlayer>() })
    }
}
