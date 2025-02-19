use core::f32;
use std::{cmp::Ordering, sync::RwLock};

use crate::sdk::{
    bindings::{BaseEntity, TFPlayer},
    class_id::ClassId,
    interface::interfaces::{self, Interfaces},
    vmts::Team,
};

pub static ESP: RwLock<Option<Esp>> = const { RwLock::new(None) };

const HP_BAR_PAD: f32 = 5.0;

pub struct Esp {
    //TODO(oxy): store different entities, a enum or something
    #[allow(clippy::type_complexity)]
    pub entities: Vec<(([f32; 2], [f32; 2], [f32; 2], [f32; 2]), i32)>,
}

impl Esp {
    //TODO(oxy):refactor this
    pub fn store_entities(&mut self) {
        let interfaces = Interfaces::get();

        if !interfaces.engine.is_in_game() {
            return;
        }

        let Some(local_player) = interfaces.engine.get_local_player() else {
            return;
        };
        self.entities.clear();
        let local_eyes = local_player.get_eye_position();

        'ent_lop: for entry in &interfaces.entity_list.cache {
            if entry.networkable.is_null() {
                continue;
            }
            if !matches!(
                (unsafe { &*entry.networkable }).get_client_class().class_id,
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

            let w2s = interfaces.client.get_w2s_matrix();

            let mut points = pos.corners(mins, maxs);

            points.sort_by(|a, b| {
                (*a - local_eyes)
                    .squared_len_2d()
                    .partial_cmp(&(*b - local_eyes).squared_len_2d())
                    .unwrap_or(Ordering::Equal)
            });

            let mut points_2d = Vec::with_capacity(4);

            for point in points.iter().skip(2).take(4) {
                let Some(point) = w2s.transform_vector(point) else {
                    continue 'ent_lop;
                };

                points_2d.push(point);
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
                player.get_entindex(),
            ));
        }
    }

    #[allow(clippy::similar_names)]
    pub fn draw(&mut self, ui: &mut imgui::Ui) {
        if !Interfaces::get().engine.is_in_game() {
            return;
        }
        let draw_list = ui.get_background_draw_list();

        let viewport = unsafe { imgui::sys::igGetMainViewport().read() };
        let window_size = [viewport.Size.x, viewport.Size.y];
        let scale = (window_size[0] as f32 / 2f32, window_size[1] as f32 / 2f32);

        for (mut corners, entindex) in &self.entities {
            let Some(ent) = Interfaces::get().entity_list.get_client_entity_from_index(*entindex) else {
                continue;
            };

            if !ent.is_alive() {
                continue;
            }
            let player = unsafe { &*std::ptr::from_ref::<BaseEntity>(ent).cast::<TFPlayer>() };

            corners.0[0] *= scale.0;
            corners.0[1] *= scale.1;

            corners.1[0] *= scale.0;
            corners.1[1] *= scale.1;

            corners.2[0] *= scale.0;
            corners.2[1] *= scale.1;

            corners.3[0] *= scale.0;
            corners.3[1] *= scale.1;

            // name
            let team_color = match player.get_team() {
                Team::Red => 0xFF_00_00_FF,
                Team::Blue => 0xFF_FF_00_00,
            };
            let info = player.get_info();
            let text_size = ui.calc_text_size(&info.name);
            draw_list.add_text(
                [
                    corners.0[0] + (corners.1[0] - corners.0[0] - text_size[0]) / 2.0,
                    corners.0[1] - text_size[1] * 2.0,
                ],
                team_color,
                info.name,
            );

            let weapon_name = {
                player
                    .get_weapon()
                    .map_or("none".to_owned(), |x| x.get_print_name())
            };
            let text_size = ui.calc_text_size(&weapon_name);
            draw_list.add_text(
                [
                    corners.0[0] + (corners.1[0] - corners.0[0] - text_size[0]) / 2.0,
                    corners.2[1] + text_size[1],
                ],
                0xFF_FF_FF_FF,
                weapon_name,
            );

            //hp bar
            let hp_bar_pos = (
                [corners.0[0] - 2.0 * HP_BAR_PAD, corners.0[1]],
                [corners.0[0] - HP_BAR_PAD, corners.2[1]],
            );

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
                    corners.0[1] - text_size[1],
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
