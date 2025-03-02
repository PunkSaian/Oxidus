use std::{
    ffi::CString,
    mem::MaybeUninit,
    ptr::{self, null},
};

use macros::{detour_hook, sig, vmt_hook};

use crate::{
    config::Config,
    hook::{detour::install_detour, vmt::install_vmt},
    i,
    math::{Angles, Vector2, Vector3},
    mdbg,
    modules::{
        aimbot::{self},
        esp::ESP,
        movement::{self, rotate_movement},
    },
    sdk::{
        bindings::BaseAnimating,
        class_id::ClassId,
        interface::{
            client::{FrameStage, ViewSetup},
            material_render_context::{MaterialCullMode, StencilComparisonFn, StencilOperation},
            material_system::{
                CompiledVtfFlags, CreateRenderTargetFlags, ImageFormat, SizeMode, TargetDepth,
            },
            model_render::{DrawModelState, ModelRender, ModelRenderInfo, OverrideType},
        },
        models::{
            bone_matrix::BoneMatrix, material::Material, model::Model, renderable::Renderable,
            texture::Texture,
        },
        module_names::CLIENT,
        texture_group_names::TEXTURE_GROUP_OTHER,
    },
};

use std::f32;

use crate::sdk::interface::client_mode::{ClientMode, UserCmd};

#[vmt_hook]
pub unsafe extern "C" fn frame_stage_notify(this: *const (), stage: FrameStage) {
    if let FrameStage::RenderEnd = stage {
        // store esp entities
        let mut esp = ESP.write().unwrap();
        if let Some(esp) = esp.as_mut() {
            esp.store_entities();
        }
    }
    original_function(this, stage);
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
    let org_cmd = *cmd;
    let overwrite_angels = !aimbot::run(cmd);

    movement::run(cmd);

    #[allow(clippy::float_cmp)]
    if org_cmd.viewangles.yaw != cmd.viewangles.yaw {
        let Vector2 { x, y } = rotate_movement(
            cmd.viewangles.yaw - org_cmd.viewangles.yaw,
            [cmd.forwardmove, cmd.sidemove].into(),
        );
        cmd.forwardmove = x;
        cmd.sidemove = y;
    }

    overwrite_angels
}

#[vmt_hook]
pub unsafe extern "C" fn override_view(this: *const (), view_setup: &mut ViewSetup) -> bool {
    let visual_settings = &Config::get_read().settings.visual;
    view_setup.fov = *visual_settings.fov.get();
    if let Some(local_player) = i!().engine.get_local_player() {
        if local_player.is_alive() {
            local_player.m_nForceTauntCam = i32::from(*visual_settings.third_person.get());
        }
    }

    original_function(this, view_setup)
}

#[vmt_hook]
pub unsafe extern "C" fn draw_model_execute(
    this: &'static ModelRender,
    state: &mut DrawModelState,
    info: &ModelRenderInfo,
    custom_bone_to_world: &BoneMatrix,
) {
    let ent = i!()
        .entity_list
        .get_client_entity_from_index(info.entity_index)
        .unwrap();
    if ent.get_class_id() == ClassId::CTFViewModel {
        mdbg!(info);
    }

    original_function(this, state, info, custom_bone_to_world);
}

#[vmt_hook]
pub unsafe extern "C" fn draw_model_ex(this: *const (), info: &ModelRenderInfo) {
    let ent = i!()
        .entity_list
        .get_client_entity_from_index(info.entity_index)
        .unwrap();
    if ent.get_class_id() == ClassId::CTFViewModel {
        mdbg!(info);
    }
    original_function(this, info);
}

#[vmt_hook]
pub unsafe extern "C" fn draw_model(
    this: &'static ModelRender,
    flags: i32,
    renderable: &'static Renderable,
    instance: i32,
    entity_index: i32,
    model: &'static Model,
    origin: Vector3,
    angles: Angles,
    skin: i32,
    body: i32,
    hitboxset: i32,
    model_to_world: &'static BoneMatrix,
    lighting_offset: &'static BoneMatrix,
) {
    let ent = i!()
        .entity_list
        .get_client_entity_from_index(entity_index)
        .unwrap();
    mdbg!(ent.get_class_id());
    original_function(
        this,
        flags,
        renderable,
        instance,
        entity_index,
        model,
        origin,
        angles,
        skin,
        body,
        hitboxset,
        model_to_world,
        lighting_offset,
    );
}

#[detour_hook]
pub unsafe extern "C" fn base_animating_internal_draw_model(
    this: &'static BaseAnimating,
    flags: i32,
) {
    if this.get_class_id() == ClassId::CTFViewModel {
        let render_context = i!().material_system.getn_render_context();
        render_context.cull_mode(MaterialCullMode::Ccw);
        render_context.clear_buffers(false, false, false);
        render_context.set_stencil_enable(true);
        render_context.set_stencil_compare_function(StencilComparisonFn::Alaway);
        render_context.set_stencil_pass_operation(StencilOperation::Replace);
        render_context.set_stencil_fail_operation(StencilOperation::Keep);
        render_context.set_stencil_zfail_operation(StencilOperation::Replace);
        render_context.set_stencil_refrence_value(1);
        render_context.set_stencil_write_mask(0xFF);
        render_context.set_stencil_test_mask(0x0);

        let old_blend = i!().engine_render_view.get_blend();
        i!().engine_render_view.set_blend(0.0);
        i!().engine_render_view
            .set_color_modulation(&[1.0, 1.0, 1.0]);
        static mut MAT_GLOW_COLOR: Option<&'static mut Material> = None;
        if MAT_GLOW_COLOR.is_none() {
            let mat = i!().material_system.find_material(
                CString::new("dev/glow_color").unwrap().as_ptr(),
                CString::new(TEXTURE_GROUP_OTHER).unwrap().as_ptr(),
                true,
                null(),
            );
            MAT_GLOW_COLOR = Some(mat);
        }
        i!().model_render
            .force_material_override(MAT_GLOW_COLOR.as_mut().unwrap(), OverrideType::Normal);

        static mut RENDER_BUFFER: Option<&'static mut Texture> = None;

        return original_function(this, 1);
        original_function(this, flags);

        let (w, h) = i!().engine.get_screen_size();
        if RENDER_BUFFER.is_none() {
            let texture = i!().material_system.create_render_target_texture_ex(
                CString::new("oxidus_glow_buffer").unwrap(),
                w,
                h,
                SizeMode::LITERAL,
                ImageFormat::RGB888,
                TargetDepth::SHARED,
                CompiledVtfFlags::CLAMPS as i64
                    | CompiledVtfFlags::CLAMPT as i64
                    | CompiledVtfFlags::EIGHTBITALPHA as i64,
                CreateRenderTargetFlags::HDR,
            );
            dbg!(std::ptr::from_mut(texture));
            RENDER_BUFFER = Some(texture);
        }

        render_context.push_render_target_and_viewport();
        render_context.set_render_target(RENDER_BUFFER.as_mut().unwrap());
        render_context.viewport(0, 0, w, h);
        render_context.clear_color_4ub(0, 0, 0, 0);
        dbg!("1");
        render_context.clear_buffers(true, false, true);
        dbg!("1");
        i!().engine_render_view
            .set_color_modulation(&[1.0, 0.0, 1.0]);
        dbg!("1");
        i!().engine_render_view.set_blend(0.5);
        dbg!("1");
        original_function(this, flags);
        dbg!("1");

        i!().engine_render_view.set_blend(old_blend);
        dbg!("1");
        render_context.cull_mode(MaterialCullMode::Cw);
        dbg!("1");
        return original_function(this, 1);
    }
    original_function(this, flags);
}

pub fn init() {
    unsafe {
        install_vmt(
            *(ptr::from_ref(i!().client).cast()),
            35,
            frame_stage_notify as *mut (),
        );
        install_vmt(
            *(ptr::from_ref(i!().client_mode).cast()),
            17,
            override_view as *mut (),
        );
        install_vmt(
            *(ptr::from_ref(i!().client_mode).cast()),
            22,
            create_move as *mut (),
        );
        install_vmt(
            *(ptr::from_ref(i!().model_render).cast()),
            19,
            draw_model_execute as *mut (),
        );
        install_vmt(
            *(ptr::from_ref(i!().model_render).cast()),
            0,
            draw_model as *mut (),
        );
        install_vmt(
            *(ptr::from_ref(i!().model_render).cast()),
            16,
            draw_model_ex as *mut (),
        );
        let base_animating_internal_draw_model_sig = sig!("55 31 C0 48 89 E5 41 57 41 56 41 55 41 89 F5 41 54 49 89 FC 53 48 81 EC 38 01 00 00 48 8B 1D 4D B3 85 01 48 C7 85 C0 FE FF");
        let base_animating_internal_draw_model_addr = base_animating_internal_draw_model_sig
            .scan_module(CLIENT)
            .unwrap();

        install_detour(
            base_animating_internal_draw_model_addr as *mut (),
            base_animating_internal_draw_model as *mut (),
        )
        .unwrap();
    }
}
