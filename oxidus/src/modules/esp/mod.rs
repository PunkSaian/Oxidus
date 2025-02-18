use core::f32;
use std::{cmp::Ordering, mem::MaybeUninit, sync::RwLock};

pub use crate::prelude::*;

use crate::{
    math::VMatrix,
    sdk::{
        bindings::{BaseEntity, TFPlayer},
        class_id::ClassId,
        interface::interfaces::Interfaces,
        vmts::Team,
    },
};

pub static ESP: RwLock<Option<Esp>> = const { RwLock::new(None) };

pub struct Esp {
    //TODO(oxy): store different entities, a enum or something
    #[allow(clippy::type_complexity)]
    pub entities: Vec<(([f32; 2], [f32; 2], [f32; 2], [f32; 2]), &'static TFPlayer)>,
}

impl Esp {
    //TODO(oxy):refactor this
    #[allow(clippy::too_many_lines, clippy::similar_names)]
    pub fn store_entities(&mut self) {
        let interfaces = INTERFACES.get().unwrap();

        self.entities.clear();

        let local_player = Interfaces::get().engine.get_local_player();
        let local_eyes = local_player.get_eye_position();

        'ent_lop: for entry in &interfaces.entity_list.cache {
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

            let player = unsafe { &*(*(networkable).get_client_unknown()).get_base_entity() };
            let pos = player.m_vecOrigin;
            let collidable = player.get_collidable();
            let mins = collidable.obb_mins();
            let maxs = collidable.obb_maxs();

            let mut view_setup = unsafe { MaybeUninit::zeroed().assume_init() };
            interfaces.client.get_player_view(&mut view_setup);

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

            let mut points = (0..8)
                .map(|i| {
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
                    pos
                })
                .collect::<Vec<_>>();

            points.sort_by(|a, b| {
                (*a - local_eyes)
                    .squared_distance_2d()
                    .partial_cmp(&(*b - local_eyes).squared_distance_2d())
                    .unwrap_or(Ordering::Equal)
            });

            let mut points_2d = Vec::with_capacity(4);

            for point in points.iter().skip(2).take(4) {
                let (screen_pos, w) = w2s.transform_vector(point);

                if w < 0.01 {
                    continue 'ent_lop;
                }

                let x = 1.0 + (screen_pos.x / w);
                let y = 1.0 - (screen_pos.y / w);
                points_2d.push([x, y]);
            }

            let mut points_2d_ltr = points_2d.clone();
            points_2d_ltr.sort_by(|a, b| a[0].partial_cmp(&b[0]).unwrap_or(Ordering::Equal));

            if points_2d_ltr[0][1] < points_2d_ltr[1][1] {
                points_2d_ltr.swap(0, 1);
            }
            if points_2d[2][1] < points_2d[3][1] {
                points_2d_ltr.swap(2, 3);
            }

            self.entities.push((
                (
                    points_2d_ltr[1],
                    points_2d_ltr[3],
                    points_2d_ltr[0],
                    points_2d_ltr[2],
                ),
                unsafe { &*std::ptr::from_ref::<BaseEntity>(player).cast::<TFPlayer>() },
            ));
        }
    }

    #[allow(clippy::similar_names)]
    pub fn draw(&mut self, ui: &mut imgui::Ui) {
        const PAD: f32 = 5.0;
        let draw_list = ui.get_background_draw_list();

        let viewport = unsafe { imgui::sys::igGetMainViewport().read() };
        let window_size = [viewport.Size.x, viewport.Size.y];

        for (mut pos, player) in &self.entities {
            if !player.is_alive() {
                continue;
            }
            let scale = (window_size[0] as f32 / 2f32, window_size[1] as f32 / 2f32);

            pos.0[0] *= scale.0;
            pos.0[1] *= scale.1;

            pos.1[0] *= scale.0;
            pos.1[1] *= scale.1;

            pos.2[0] *= scale.0;
            pos.2[1] *= scale.1;

            pos.3[0] *= scale.0;
            pos.3[1] *= scale.1;

            // name
            let team_color = match player.get_team() {
                Team::Red => 0xFF_00_00_FF,
                Team::Blue => 0xFF_FF_00_00,
            };
            let info = player.get_info();
            let text_size = ui.calc_text_size(&info.name);
            draw_list.add_text(
                [
                    pos.0[0] + (pos.1[0] - pos.0[0] - text_size[0]) / 2.0,
                    pos.0[1] - text_size[1],
                ],
                team_color,
                info.name,
            );

            //weapon
            let weapon_name = player.get_weapon().get_print_name();
            let text_size = ui.calc_text_size(&weapon_name);
            draw_list.add_text(
                [
                    pos.0[0] + (pos.1[0] - pos.0[0] - text_size[0]) / 2.0,
                    pos.2[1] + text_size[1],
                ],
                0xFF_FF_FF_FF,
                weapon_name,
            );

            //hp bar
            let hp_bar_pos = ([pos.0[0] - 2.0 * PAD, pos.0[1]], [pos.0[0] - PAD, pos.2[1]]);

            let hp = player.m_iHealth as f32;
            let max_hp = player.get_max_health() as f32;
            draw_list
                .add_rect(hp_bar_pos.0, hp_bar_pos.1, 0xFF_00_00_00)
                .thickness(1.0)
                .filled(true)
                .build();

            let mut hp_color = 0xFF_00_FF_00;
            let mut hp_ratio = hp / max_hp;

            if hp_ratio < 0.2 {
                hp_color = 0xFF_00_00_FF;
            } else if hp_ratio > 1.0 {
                draw_list
                    .add_rect(hp_bar_pos.0, hp_bar_pos.1, hp_color)
                    .thickness(1.0)
                    .filled(true)
                    .build();
                hp_ratio -= 0.5;
                hp_color = 0xFF_FF_00_00;
            }

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

            // hp text
            let hp_text = format!("{hp}/{max_hp}");
            let text_size = ui.calc_text_size(&hp_text);

            draw_list.add_text(
                [
                    hp_bar_pos.0[0] + (hp_bar_pos.1[0] - hp_bar_pos.0[0] - text_size[0]) / 2.0,
                    pos.0[1] - text_size[1],
                ],
                0xFF_FF_FF_FF,
                hp_text,
            );
        }
    }
}

pub fn init() {
    let esp = Esp { entities: vec![] };
    *ESP.write().unwrap() = Some(esp);
}
