use std::{
    mem::{transmute, MaybeUninit},
    sync::RwLock,
};

use imgui::sys;

pub use crate::prelude::*;

use crate::{
    math::VMatrix,
    mdbg, mdbg_input,
    sdk::{
        bindings::{BaseEntity, TFPlayer},
        class_id::ClassId,
        interface::interfaces::Interfaces,
        networkable,
    },
};

pub static ESP: RwLock<Option<Esp>> = const { RwLock::new(None) };

pub struct Esp {
    pub entities: Vec<(([f32; 2], [f32; 2]), &'static TFPlayer)>,
}

impl Esp {
    pub fn store_entities(&mut self) {
        let interfaces = INTERFACES.get().unwrap();

        self.entities.clear();

        let local_player_entindex = Interfaces::get().engine.get_loacl_player_entindex();

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

            let networkable = unsafe { &*entry.networkable };
            if networkable.get_index() == local_player_entindex {
                continue;
            }

            let player = unsafe { &*(*(networkable).get_client_unknown()).get_base_entity() };
            let pos = player.m_vecOrigin;
            let collidable = player.get_collidable();
            let mins = collidable.obb_mins();
            let maxs = collidable.obb_maxs();

            let mut view_setup = unsafe { MaybeUninit::zeroed().assume_init() };
            interfaces.client.get_player_view(&mut view_setup);

            let mut screen_w = 0;
            let mut screen_h = 0;

            mdbg!((screen_w, screen_h));

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

            let mut min = [f32::INFINITY, f32::INFINITY];
            let mut max = [f32::NEG_INFINITY, f32::NEG_INFINITY];

            for i in 0..8 {
                let mut pos = pos;
                if i & 1 == 0 {
                    pos.x += mins.x;
                } else {
                    pos.x += maxs.x;
                }
                if i & 2 == 0 {
                    pos.y += mins.y;
                } else {
                    pos.y += maxs.y;
                }
                if i & 4 == 0 {
                    pos.z += mins.z;
                } else {
                    pos.z += maxs.z;
                }

                let (screen_pos, w) = w2s.transform_vector(&pos);

                if w < 0.01 {
                    continue;
                }

                let x = screen_pos.x / w;
                let y = 0.0 - (screen_pos.y / w);

                if x < min[0] {
                    min[0] = x;
                }
                if y < min[1] {
                    min[1] = y;
                }
                if x > max[0] {
                    max[0] = x;
                }
                if y > max[1] {
                    max[1] = y;
                }
            }
            if min[0] == f32::INFINITY || min[1] == f32::INFINITY || max[0] == f32::NEG_INFINITY {
                continue;
            }

            self.entities
                .push(((min, max), unsafe { std::mem::transmute(player) }));
        }
    }

    #[allow(clippy::similar_names)]
    pub fn draw(&mut self, ui: &mut imgui::Ui) {
        const PAD: f32 = 5.0;
        let draw_list = ui.get_background_draw_list();

        let viewport = unsafe { imgui::sys::igGetMainViewport().read() };
        let window_size = [viewport.Size.x, viewport.Size.y];

        for (mut pos, player) in &self.entities {
            let ent_index = player.get_index();
            let info = Interfaces::get().engine.get_player_info(ent_index);
            let text_size = ui.calc_text_size(&info.name);

            pos.0[0] = (window_size[0] as f32 / 2f32) * (1f32 + pos.0[0]);
            pos.0[1] = (window_size[1] as f32 / 2f32) * (1f32 + pos.0[1]);

            pos.1[0] = (window_size[0] as f32 / 2f32) * (1f32 + pos.1[0]);
            pos.1[1] = (window_size[1] as f32 / 2f32) * (1f32 + pos.1[1]);

            draw_list.add_text(
                [
                    pos.0[0] + (pos.1[0] - pos.0[0] - text_size[0]) / 2.0,
                    pos.0[1] - text_size[1],
                ],
                0xFF_FF_FF_FF,
                info.name,
            );

            let hp_bar_pos = ([pos.0[0] - 2.0 * PAD, pos.0[1]], [pos.0[0] - PAD, pos.1[1]]);

            let hp = player.m_iHealth;
            let weapon = player.get_weapon();
            mdbg!(weapon.get_print_name());
            //mdbg!(unsafe { &*weapon.as_networkable().get_client_class() }.parse());
            let mut hp_color = 0xFF_00_FF_00;
            let mut hp_ratio = hp as f32 / player.get_max_health() as f32;

            if hp_ratio < 0.2 {
                hp_color = 0xFF_FF_00_00;
            } else if hp_ratio > 1.0 {
                hp_ratio -= 1.0;
                hp_color = 0xFF_00_00_FF;
            }

            draw_list
                .add_rect(hp_bar_pos.0, hp_bar_pos.1, 0xFF_00_00_00)
                .thickness(1.0)
                .filled(true)
                .build();

            let hp_bar_pos_top = [
                hp_bar_pos.0[0],
                hp_bar_pos.1[1] - (hp_bar_pos.1[1] - hp_bar_pos.0[1]) * hp_ratio,
            ];

            draw_list
                .add_rect(hp_bar_pos_top, hp_bar_pos.1, hp_color)
                .thickness(1.0)
                .filled(true)
                .build();

            draw_list
                .add_rect(hp_bar_pos.0, hp_bar_pos.1, 0xFF_00_00_00)
                .thickness(1.0)
                .build();
        }
    }
}

pub fn init() {
    let esp = Esp { entities: vec![] };
    *ESP.write().unwrap() = Some(esp);
}
