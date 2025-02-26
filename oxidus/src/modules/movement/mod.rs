use std::ptr;

use crate::{
    config::Config,
    i,
    math::{Vector2, Vector3},
    sdk::{
        bindings::{LocalPlayerExclusive, TFPlayer},
        interface::client_mode::{ButtonFlags, UserCmd},
        models::tf_player::{Flag, PlayerClass},
    },
};

pub fn init() {}

pub fn rotate_movement(yaw: f32, vec: Vector2) -> Vector2 {
    let alpha = yaw.to_radians();

    [
        vec.x * alpha.cos() - vec.y * alpha.sin(),
        vec.y * alpha.cos() + vec.x * alpha.sin(),
    ]
    .into()
}

pub fn run(cmd: &mut UserCmd) {
    static mut JUMPED_LAST: bool = false;
    static mut DOUBLLE_JUMPED: bool = false;
    let Some(local_player) = i!().engine.get_local_player() else {
        return;
    };

    if !local_player.is_alive() || local_player.get_flags().get(Flag::Swim) {
        return;
    }

    let jumping = cmd.buttons.get(ButtonFlags::InJump);
    unsafe {
        if local_player.get_flags().get(Flag::Onground) {
            DOUBLLE_JUMPED = false;
        } else if matches!(*local_player.get_class(), PlayerClass::Scout)
            && jumping
            && !DOUBLLE_JUMPED
            && !JUMPED_LAST
        {
            DOUBLLE_JUMPED = true;
        } else {
            bhop(cmd);
            auto_strafe(cmd);
        }
        JUMPED_LAST = jumping;
    }

    momentum_compensation(cmd);
}

pub fn momentum_compensation(cmd: &mut UserCmd) {
    let movement_settings = &Config::get_read().settings.movement;
    if !movement_settings.momentum_compensation.get() {
        return;
    }

    let Some(local_player) = i!().engine.get_local_player() else {
        return;
    };

    let local_data =
        unsafe { &*ptr::from_ref::<TFPlayer>(local_player).cast::<LocalPlayerExclusive>() };
    let vel = Vector3::from(local_data.m_vecVelocity);
    let rotate_vel = rotate_movement(180.0 - cmd.viewangles.yaw, Vector2::from([vel.x, vel.y]));
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
    let movement_settings = &Config::get_read().settings.movement;
    if !movement_settings.bhop.get() {
        return;
    }

    let Some(local_player) = i!().engine.get_local_player() else {
        return;
    };

    let on_ground = local_player.get_flags().get(Flag::Onground);
    cmd.buttons.set(
        ButtonFlags::InJump,
        cmd.buttons.get(ButtonFlags::InJump) && on_ground,
    );
}

pub fn auto_strafe(cmd: &mut UserCmd) {
    const SPEED_VAR: f32 = 6062.0;
    const WISH_SPEED: f32 = 30.0;
    let movement_settings = &Config::get_read().settings.movement;
    if !movement_settings.auto_strafe.get() {
        return;
    }

    let Some(local_player) = i!().engine.get_local_player() else {
        return;
    };

    if !local_player.is_alive() {
        return;
    }

    if local_player.get_flags().get(Flag::Onground) {
        return;
    }
    if cmd.forwardmove == 0.0 && cmd.sidemove == 0.0 {
        return;
    }

    let local_data =
        unsafe { &*ptr::from_ref::<TFPlayer>(local_player).cast::<LocalPlayerExclusive>() };

    let velocity = Vector3::from(local_data.m_vecVelocity);
    let speed = velocity.len_2d();

    let air_accelerate = i!().engine_cvar.get_cvar("sv_airaccelerate").float_value;

    let term = WISH_SPEED / air_accelerate / SPEED_VAR * 100.0 / speed;

    let perfect_delta = if -1.0 < term && term < 1.0 {
        term.acos()
    } else {
        0.0
    };

    let desired_angle = (-cmd.sidemove).atan2(cmd.forwardmove);
    let yaw = cmd.viewangles.yaw.to_radians();
    let angle = velocity.y.atan2(velocity.x) - yaw;
    let mut delta = angle - desired_angle;
    while delta > std::f32::consts::PI {
        delta -= 2.0 * std::f32::consts::PI;
    }
    while delta < -std::f32::consts::PI {
        delta += 2.0 * std::f32::consts::PI;
    }

    let direction = if delta < 0.0 {
        angle + perfect_delta
    } else {
        angle - perfect_delta
    };

    cmd.forwardmove = direction.cos() * 450.0;
    cmd.sidemove = -direction.sin() * 450.0;
}
