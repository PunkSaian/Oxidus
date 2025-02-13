use std::{mem::MaybeUninit, usize};

use imgui::DrawListMut;
use macros::vmt_hook;

use crate::{
    math::VMatrix,
    sdk::{
        bindings::Vector2,
        class_id::ClassId,
        interface::{
            client::{self, Client, ViewSetup},
            client_entity_list::ClientEntityList,
            engine::{self, Engine},
            engine_render_view::EngineRenderView,
            interface_names,
        },
        module_names,
    },
    util::create_interface,
};

pub mod viewmatrix;

pub fn init_esp() {}

pub fn draw(ui: &mut imgui::Ui) {
    let entity_list = create_interface::<ClientEntityList>(
        module_names::CLIENT,
        interface_names::CLIENT_ENTITY_LIST,
    )
    .unwrap();

    let client = create_interface::<Client>(module_names::CLIENT, interface_names::CLIENT).unwrap();

    let engine = create_interface::<Engine>(module_names::ENGINE, interface_names::ENGINE).unwrap();
    let engine_render_view = create_interface::<EngineRenderView>(
        module_names::ENGINE,
        interface_names::ENGINE_RENDER_VIEW,
    )
    .unwrap();

    let mut view_setup = unsafe { MaybeUninit::uninit().assume_init() };
    client.get_player_view(&mut view_setup);

    let mut screen_w = 0;
    let mut screen_h = 0;
    engine.get_screen_size(&mut screen_w, &mut screen_h);

    let mut w2v: VMatrix = unsafe { MaybeUninit::zeroed().assume_init() };
    let mut v2pr: VMatrix = unsafe { MaybeUninit::zeroed().assume_init() };
    let mut w2s: VMatrix = unsafe { MaybeUninit::zeroed().assume_init() };
    let mut w2px: VMatrix = unsafe { MaybeUninit::zeroed().assume_init() };
    engine_render_view.get_marices_for_view(&view_setup, &mut w2v, &mut v2pr, &mut w2s, &mut w2px);

    let draw_list = ui.get_background_draw_list();

    for entry in &entity_list.cache {
        if entry.networkable.is_null() {
            continue;
        }
        if !matches!(
            unsafe { &*(&*entry.networkable).get_client_class() }.class_id,
            ClassId::CTFPlayer
        ) {
            continue;
        };

        let player = unsafe { &*(&*(&*entry.networkable).get_client_unknown()).get_base_entity() };
        let player_pos = player.m_vecOrigin;

        let w = w2s.origin.vec.dot(&player_pos) + w2s.origin.w;
        let x = w2s.right.vec.dot(&player_pos) + w2s.right.w;
        let y = w2s.up.vec.dot(&player_pos) + w2s.up.w;

        if w < 0.01 {
            continue;
        }

        let x = screen_w as f32 / 2f32 * (1f32 + x / w);
        let y = screen_h as f32 / 2f32 * (1f32 - y / w);

        draw_list.add_circle([x, y], 50.0, [1.0, 0.0, 0.0]).build();
    }
}
