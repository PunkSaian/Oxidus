use std::collections::HashMap;

use imgui::{Id, Key, TableFlags, WindowFlags};

use crate::{
    config::{binds::Bind, diff_settings, Config},
    sdk::bindings,
};

#[allow(static_mut_refs, clippy::too_many_lines)]
pub fn show_binds(ui: &mut imgui::Ui) {
    let config = Config::get();
    let mut config = config.write().unwrap();
    pub static mut WAITING_FOR_KEYS: bool = false;
    pub static mut PRESED_KEYS: Vec<Key> = Vec::new();
    pub static mut NAME: String = String::new();
    ui.modal_popup_config("new bind")
        .resizable(false)
        .movable(false)
        .always_auto_resize(true)
        .build(|| unsafe {
            //Name
            ui.input_text("Name", &mut NAME).build();

            //Bind button
            if WAITING_FOR_KEYS {
                let prev_keys = PRESED_KEYS.clone();
                PRESED_KEYS.clear();
                for key in Key::VARIANTS {
                    if ui.is_key_down(key) {
                        PRESED_KEYS.push(key);
                    }
                }
                if PRESED_KEYS.len() < prev_keys.len() {
                    PRESED_KEYS = prev_keys;
                    WAITING_FOR_KEYS = false;
                }
            }
            let button_name = if PRESED_KEYS.is_empty() {
                if WAITING_FOR_KEYS {
                    "..".to_owned()
                } else {
                    "select keys".to_owned()
                }
            } else {
                PRESED_KEYS
                    .iter()
                    .map(|x| format!("{x:?}"))
                    .collect::<Vec<String>>()
                    .join(" + ")
            };
            if ui.button(button_name) && !WAITING_FOR_KEYS {
                PRESED_KEYS.clear();
                WAITING_FOR_KEYS = true;
            }

            //----
            ui.disabled(PRESED_KEYS.is_empty() || NAME.is_empty(), || {
                if ui.button("create") {
                    config.binds.push(Bind {
                        name: NAME.clone(),
                        keys: PRESED_KEYS.clone(),
                        diff: HashMap::new(),
                        triggered: false,
                    });
                    ui.close_current_popup();
                }
            });
            ui.same_line();
            if ui.button("cancel") {
                ui.close_current_popup();
            }
        });

    let popup_id = ui.new_id_str("new bind");
    let open_popup = || unsafe {
        WAITING_FOR_KEYS = false;
        PRESED_KEYS.clear();
        NAME.clear();
        imgui::sys::igOpenPopup_ID(std::mem::transmute::<Id, u32>(popup_id), 0);
    };
    #[allow(clippy::redundant_pattern_matching)]
    ui.window("Binds")
        .flags(
            WindowFlags::NO_DOCKING
                | WindowFlags::NO_COLLAPSE
                | WindowFlags::ALWAYS_AUTO_RESIZE
                | WindowFlags::NO_RESIZE,
        )
        .build(|| {
            if let Some(_) = ui.begin_table("binds", 4) {
                ui.table_setup_column("Name");
                ui.table_setup_column("Trigger");
                ui.table_setup_column("Settings");
                ui.table_setup_column("Options");
                ui.table_headers_row();

                for (i, bind) in config.binds.clone().iter().enumerate() {
                    ui.table_next_row();
                    ui.table_next_column();
                    ui.text(&bind.name);
                    ui.table_next_column();
                    ui.text(
                        bind.keys
                            .iter()
                            .map(|x| format!("{x:?}"))
                            .collect::<Vec<_>>()
                            .join(" + "),
                    );
                    ui.table_next_column();
                    let commands_string = bind
                        .diff
                        .iter()
                        .map(|x| format!("{} = {:?}", x.0.join("->"), x.1))
                        .collect::<Vec<String>>()
                        .join("\n");
                    ui.text(&commands_string);
                    ui.table_next_column();
                    ui.disabled(config.binding.is_some(), || {
                        if ui.button(format!("Edit###{i}")) {
                            config.binding = Some((i, config.settings.clone()));
                            bind.apply(&mut config);
                        }
                    });
                    if let Some(binding) = config.binding.clone() {
                        ui.same_line();
                        if binding.0 == i && ui.button("Save") {
                            let old_settings = config.binding.as_ref().unwrap().clone().1;

                            let diff = diff_settings(&old_settings, &config.settings);
                            //compare new settings state with saved one and save if different
                            config.settings = old_settings;
                            config.binding = None;
                            config.binds[i].diff = diff;
                        }
                    }
                }
            }

            if ui.button("New") {
                open_popup();
            }
        });
}
