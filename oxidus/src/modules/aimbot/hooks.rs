use macros::{detour_hook, vmt_hook};

use crate::{
    math::Vector2,
    mdbg,
    sdk::{
        bindings::{BaseEntity, TFPlayer},
        class_id::ClassId,
        interface::{
            client_mode::{ClientMode, UserCmd},
            interfaces::Interfaces,
            model_info::PlayerHitboxId,
        },
    },
    Vector3,
};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RotationVectors {
    pub forward: Vector3,
    pub right: Vector3,
    pub up: Vector3,
}

pub fn dtr(deg: f32) -> f32 {
    (deg / 180f32) * std::f32::consts::PI
}

pub fn rotate_movement(yaw: f32, vec: Vector2) -> Vector2 {
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
    let org_cmd = *cmd;

    let local_player = Interfaces::get().engine.get_local_player();

    let local_eyes = local_player.get_eye_position();

    'ent_lop: for entry in &Interfaces::get().entity_list.cache {
        if entry.networkable.is_null() {
            continue;
        }
        if !matches!(
            unsafe { &*(*entry.networkable).get_client_class() }.class_id,
            ClassId::CTFPlayer
        ) {
            continue;
        }

        let networkable = unsafe { &*entry.networkable };
        if networkable.get_index() == local_player.get_entindex() {
            continue;
        }

        let ent = unsafe { &*(*(networkable).get_client_unknown()).get_base_entity() };
        let player = &*std::ptr::from_ref::<BaseEntity>(ent).cast::<TFPlayer>();
        let pelvis = player
            .as_renderable()
            .get_hitboxes()
            .get_hitbox(*PlayerHitboxId::Pelvis);
        let bone =
            player.as_renderable().get_hitbox_bones()[PlayerHitboxId::Pelvis as i32 as usize];
        let pos: Vector3 = [bone.0[0][3], bone.0[1][3], bone.0[2][3]].into();
        let angle = RotationVectors {
            forward: [bone.0[0][0], bone.0[0][1], bone.0[0][2]].into(),
            right: [bone.0[1][0], bone.0[1][1], bone.0[1][2]].into(),
            up: [bone.0[2][0], bone.0[2][1], bone.0[2][2]].into(),
        };

        let diff = pos - local_eyes;

        let angle = diff.angle();

        cmd.viewangles = angle;
        mdbg!(angle);
    }
    if org_cmd.viewangles.yaw != cmd.viewangles.yaw {
        let Vector2 { x, y } = rotate_movement(
            &cmd.viewangles.yaw - &org_cmd.viewangles.yaw,
            [cmd.forwardmove, cmd.sidemove].into(),
        );
        cmd.forwardmove = x;
        cmd.sidemove = y;
    }

    false
}
