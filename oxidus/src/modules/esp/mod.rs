use std::mem::MaybeUninit;

pub use crate::prelude::*;

use crate::{math::VMatrix, mdbg, sdk::class_id::ClassId};

pub fn init_esp() {}

#[allow(clippy::similar_names)]
pub fn draw(ui: &mut imgui::Ui) {
    let interfaces = INTERFACES.get().unwrap();

    let mut view_setup = unsafe { MaybeUninit::zeroed().assume_init() };
    interfaces.client.get_player_view(&mut view_setup);

    let mut screen_w = 0;
    let mut screen_h = 0;
    interfaces
        .engine
        .get_screen_size(&mut screen_w, &mut screen_h);

    let mut w2v: VMatrix = unsafe { MaybeUninit::zeroed().assume_init() };
    let mut v2pr: VMatrix = unsafe { MaybeUninit::zeroed().assume_init() };

    let mut w2s: VMatrix = unsafe { MaybeUninit::zeroed().assume_init() };
    let mut w2px: VMatrix = unsafe { MaybeUninit::zeroed().assume_init() };
    interfaces.engine_render_view.get_marices_for_view(
        &view_setup,
        &mut w2v,
        &mut v2pr,
        &mut w2s,
        &mut w2px,
    );

    let draw_list = ui.get_background_draw_list();

    for entry in &interfaces.entity_list.cache {
        if entry.networkable.is_null() {
            continue;
        }
        if !matches!(
            unsafe { &*(*entry.networkable).get_client_class() }.class_id,
            ClassId::CTFPlayer
        ) {
            continue;
        }

        let player = unsafe { &*(*(*entry.networkable).get_client_unknown()).get_base_entity() };
        let player_pos = player.m_vecOrigin;

        let (screen_pos, w) = w2s.transform_vector(&player_pos);

        if w < 0.01 {
            continue;
        }
        mdbg!((screen_pos, w));

        let x = screen_w as f32 / 2f32 * (1f32 + screen_pos.x / w);
        let y = screen_h as f32 / 2f32 * (1f32 - screen_pos.y / w);

        draw_list.add_rect([x, y], [x, y], [1.0, 0.0, 0.0]).build();
    }
}
