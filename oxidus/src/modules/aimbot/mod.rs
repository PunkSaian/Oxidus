use crate::{
    config::Config,
    i,
    math::Vector3,
    mdbg_angle, mdbg_point,
    overlay::Interfaces,
    sdk::{
        class_id::ClassId,
        interface::{
            client_mode::{ButtonFlags, UserCmd},
            engine_trace::{CONTENTS_GRATE, MASK_SHOT},
        },
    },
};

pub fn init() {}

pub fn run(cmd: &mut UserCmd) -> bool {
    let aimbot_settings = &Config::get_read().settings.aimbot;

    if !aimbot_settings.enabled.get() {
        return false;
    }

    let Some(local_player) = i!().engine.get_local_player() else {
        return false;
    };

    if !local_player.is_alive() {
        return false;
    }

    let local_eyes = local_player.get_eye_position();
    let view_setup = i!().client.get_player_view();

    let forward = cmd.viewangles.forward();

    for player in i!()
        .entity_list
        .iterate_valid_entities(&[ClassId::CTFPlayer])
    {
        if player.get_entindex() == local_player.get_entindex() {
            continue;
        }

        if player.get_team() == local_player.get_team() {
            continue;
        }

        let bones = player.as_renderable().get_hitbox_bones();

        let hitboxes = player.as_renderable().get_hitboxes();

        //INFO: temp till peropper hitbox choing
        for i in 1..=17 {
            let hitbox = hitboxes.get_hitbox(i);

            let bone = bones[hitbox.bone as usize];

            let mut pos = bone.position();

            let rotation = bone.rotation();

            pos += ((hitbox.max + hitbox.min) / 2.0).rotate(&rotation);

            let eyes_diff = pos - local_eyes;
            let camera_diff = pos - view_setup.origin;

            let Some(camera_diff_normalized) = camera_diff.normalized() else {
                continue;
            };
            let fov_threshold = aimbot_settings.fov.get().to_radians().cos();
            if forward.dot(&camera_diff_normalized) < fov_threshold {
                continue;
            }

            let trace =
                i!().engine_trace
                    .trace(local_player, local_eyes, pos, MASK_SHOT | CONTENTS_GRATE);
            if trace.entity != player {
                continue;
            }
            cmd.buttons.set(ButtonFlags::InAttack, true);
            let mut angle = eyes_diff.angle();
            cmd.viewangles = angle;

            angle.pitch = 0.0;
            mdbg_point!("target", pos);
            mdbg_angle!("real", local_player.m_vecOrigin, angle);
            return true;
        }
    }

    false
}
