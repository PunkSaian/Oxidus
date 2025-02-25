use crate::{
    config::Config,
    get_setting, i,
    mdbg_angle, mdbg_point,
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
    let mut config = Config::get();
    if !get_setting!(&mut config.settings, "aimbot", "enabled" => Bool) {
        return false;
    }

    let Some(local_player) = i!().engine.get_local_player() else {
        return false;
    };

    if !local_player.is_alive() {
        return false;
    }

    let local_eyes = local_player.get_eye_position();

    let forward = cmd.viewangles.forward();

    let fov = get_setting!(&mut config.settings, "aimbot", "fov" => F32);

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

        for i in 0..=17 {
            let hitbox = hitboxes.get_hitbox(i);

            let bone = bones[hitbox.bone as usize];

            let mut pos = bone.position();

            let rotation = bone.rotation();

            pos += ((hitbox.max + hitbox.min) / 2.0).rotate(&rotation);

            let diff = pos - local_eyes;

            let Some(diff_normalized) = diff.normalized() else{
                continue
            };

            let dot = forward.dot(&diff_normalized);
            let fov_threshold = fov.to_radians().cos();

            let trace =
                i!().engine_trace
                    .trace(local_player, local_eyes, pos, MASK_SHOT | CONTENTS_GRATE);
            if trace.entity != player {
                continue;
            }
            if dot < fov_threshold {
                continue;
            }
            cmd.buttons.set(ButtonFlags::InAttack, true);
            let mut angle = diff.angle();
            cmd.viewangles = angle;

            angle.pitch = 0.0;
            mdbg_point!("target", pos);
            mdbg_angle!("real", local_player.m_vecOrigin, angle);
            return true;
        }
    }

    false
}
