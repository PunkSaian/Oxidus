use macros::detour_hook;

use crate::sdk::interface::client_mode::{ClientMode, UserCmd};

#[detour_hook]
pub unsafe extern "C" fn create_move(
    client_mode: &ClientMode,
    input_sample_time: f32,
    cmd: &mut UserCmd,
) -> bool {
    dbg!(input_sample_time);
    dbg!(&cmd);
    let res = original_function(client_mode, input_sample_time, cmd);
    dbg!(cmd);
    dbg!(res);
    res
}
