use std::f32;

use crate::{
    get_entry_mut, mdbg,
    overlay::AIMBOT,
    sdk::interface::{
        client_mode::ButtonFlags,
        engine_trace::{CONTENTS_GRATE, MASK_SHOT},
    },
    settings::Settings,
};
use macros::vmt_hook;

use crate::{
    math::Vector2,
    sdk::{
        bindings::{BaseEntity, TFPlayer},
        class_id::ClassId,
        interface::{
            client_mode::{ClientMode, UserCmd},
            interfaces::Interfaces,
        },
    },
};

pub fn dtr(deg: f32) -> f32 {
    (deg / 180f32) * std::f32::consts::PI
}

pub fn rotate_movement(yaw: f32, vec: &Vector2) -> Vector2 {
    let alpha = dtr(yaw);

    [
        vec.x * alpha.cos() - vec.y * alpha.sin(),
        vec.y * alpha.cos() + vec.x * alpha.sin(),
    ]
    .into()
}

#[vmt_hook]
pub unsafe extern "C" fn create_move(
    client_mode: &ClientMode,
    input_sample_time: f32,
    cmd: &mut UserCmd,
) -> bool {
    let org_res = original_function(client_mode, input_sample_time, cmd);
    if cmd.tick_count == 0 {
        return org_res;
    }

    if !AIMBOT {
        return org_res;
    }

    let Some(local_player) = Interfaces::get().engine.get_local_player() else {
        return org_res;
    };

    if !local_player.is_alive() {
        return org_res;
    }

    let org_cmd = *cmd;
    let local_eyes = local_player.get_eye_position();

    let forward = org_cmd.viewangles.forward();

    let settings = Settings::get();
    let mut settings = settings.write().unwrap();
    let fov = get_entry_mut!(&mut settings.config, "aimbot", "fov" => F32);

    'ent_loop: for entry in &Interfaces::get().entity_list.cache {
        if entry.networkable.is_null() {
            continue;
        }
        let networkable = unsafe { &*entry.networkable };
        if !matches!(networkable.get_client_class().class_id, ClassId::CTFPlayer)
            || networkable.is_dormant()
        {
            continue;
        }

        if networkable.get_index() == local_player.get_entindex() {
            continue;
        }

        let ent = unsafe { &*(*(networkable).get_client_unknown()).get_base_entity() };
        let player = &*std::ptr::from_ref::<BaseEntity>(ent).cast::<TFPlayer>();

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
            let fov_threshold = dtr(*fov).cos();

            let trace = Interfaces::get().engine_trace.trace(
                local_player,
                local_eyes,
                pos,
                MASK_SHOT | CONTENTS_GRATE,
            );
            if trace.entity != ent {
                continue;
            }
            if dot < fov_threshold {
                continue;
            }
            cmd.buttons.set(ButtonFlags::InAttack, true);
            let angle = diff.angle();
            cmd.viewangles = angle;

            break 'ent_loop;
        }
    }

    #[allow(clippy::float_cmp)]
    if org_cmd.viewangles.yaw != cmd.viewangles.yaw {
        let Vector2 { x, y } = rotate_movement(
            cmd.viewangles.yaw - org_cmd.viewangles.yaw,
            &[cmd.forwardmove, cmd.sidemove].into(),
        );
        cmd.forwardmove = x;
        cmd.sidemove = y;
    }

    false
}
