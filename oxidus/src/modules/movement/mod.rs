use std::{mem::transmute, ptr};

use crate::{
    config::Config,
    get_setting, i,
    math::{Vector2, Vector3},
    sdk::{
        bindings::{LocalPlayerExclusive, TFPlayer},
        interface::client_mode::{ButtonFlags, UserCmd},
        models::tf_player::Flag,
    },
};

pub fn init() {}

pub fn rotate_movement(yaw: f32, vec: &Vector2) -> Vector2 {
    let alpha = yaw.to_radians();

    [
        vec.x * alpha.cos() - vec.y * alpha.sin(),
        vec.y * alpha.cos() + vec.x * alpha.sin(),
    ]
    .into()
}

pub fn run(cmd: &mut UserCmd) {
    momentum_compensation(cmd);
    bhop(cmd);
}

pub fn momentum_compensation(cmd: &mut UserCmd) {
    let mut config = Config::get();
    if !get_setting!(&mut config.settings, "movement", "momentum_compensation" => Bool) {
        return;
    }

    let Some(local_player) = i!().engine.get_local_player() else {
        return;
    };

    if !local_player.is_alive() {
        return;
    }
    let local_data =
        unsafe { &*ptr::from_ref::<TFPlayer>(local_player).cast::<LocalPlayerExclusive>() };
    let vel = Vector3::from(local_data.m_vecVelocity);
    let rotate_vel = rotate_movement(180.0 - cmd.viewangles.yaw, &Vector2::from([vel.x, vel.y]));
    let friction = local_data.m_flFriction;
    let drop = rotate_vel * friction * i!().global_vars.frametime;

    if cmd.forwardmove == 0.0 {
        cmd.forwardmove = rotate_vel.x - drop.x;
    }
    if cmd.sidemove == 0.0 {
        cmd.sidemove = -rotate_vel.y + drop.y;
    }
}
pub fn bhop(cmd: &mut UserCmd) {
    let mut config = Config::get();
    if !get_setting!(&mut config.settings, "movement", "bhop" => Bool) {
        return;
    }

    let Some(local_player) = i!().engine.get_local_player() else {
        return;
    };

    if !local_player.is_alive() {
        return;
    }
    let on_ground = local_player.get_flags().get(Flag::Onground);
    cmd.buttons.set(
        ButtonFlags::InJump,
        cmd.buttons.get(ButtonFlags::InJump) && on_ground,
    );
}
