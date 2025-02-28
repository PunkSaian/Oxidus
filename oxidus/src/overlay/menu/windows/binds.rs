use imgui::{Id, Key, WindowFlags};

use crate::config::{binds::Bind, Config};

#[allow(static_mut_refs, clippy::too_many_lines)]
pub fn show_binds(ui: &mut imgui::Ui) {
    pub static mut WAITING_FOR_KEYS: bool = false;
    pub static mut PRESED_KEYS: Vec<Key> = Vec::new();
    pub static mut NAME: String = String::new();
    let mut config = Config::get_write();
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

            ui.disabled(PRESED_KEYS.is_empty() || NAME.is_empty(), || {
                if ui.button("create") {
                    config.binds.binds.push(Bind::new(&NAME, &PRESED_KEYS));
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
        .flags(WindowFlags::NO_DOCKING | WindowFlags::NO_COLLAPSE | WindowFlags::ALWAYS_AUTO_RESIZE)
        .build(|| {
            if let Some(table) = ui.begin_table("binds", 3) {
                ui.table_setup_column("Name");
                ui.table_setup_column("Trigger");
                ui.table_setup_column("Options");
                ui.table_headers_row();

                for (i, bind) in config.binds.binds.clone().iter().enumerate() {
                    ui.table_next_row();
                    ui.table_next_column();
                    ui.text(&bind.name);
                    ui.table_next_column();
                    ui.text(bind.get_keys_string());
                    ui.table_next_column();
                    ui.disabled(config.binding.is_some(), || {
                        if ui.button(format!("Settings##{i}")) {
                            config.binding = Some((i, config.settings.clone()));
                            config.settings.apply_overwrites_permanent(&bind.overwrites);
                        }
                        ui.same_line();
                        if ui.button(format!("Edit##{i}")) {
                            //TODO(oxy): reopen the popup with the bind data
                        }
                        ui.same_line();
                        if ui.button(format!("Delete##{i}")) {
                            config.binds.binds.remove(i);
                        }
                    });
                }
                table.end();
            }

            if let Some(binding) = config.binding.clone() {
                if let Some(table) = ui.begin_table("settings", 3) {
                    ui.table_setup_column("Path");
                    ui.table_setup_column("Value");
                    ui.table_setup_column("Options");
                    ui.table_next_row();
                    //for (i, (path, value)) in config.binds.binds[binding.0]
                    //    .diff
                    //    .clone()
                    //    .iter()
                    //    .enumerate()
                    //{
                    //    ui.table_next_column();
                    //    ui.text(path.join("->"));
                    //    ui.table_next_column();
                    //    ui.text(format!("{value:?}"));
                    //    ui.table_next_column();
                    //    if ui.button(format!("Delete##{i}")) {
                    //        config.binds.binds[binding.0].diff.remove(path);
                    //    }
                    //}
                    table.end();
                }
                if ui.button("Save") {
                    let old_settings = config.binding.as_ref().unwrap().clone().1;
                    let mut overwrites = old_settings.clone();
                    overwrites.generate_overwrites(&config.settings);
                    config.binds.binds[binding.0].overwrites = overwrites;
                    config.settings = old_settings;
                    config.binding = None;
                }
                ui.same_line();
                if ui.button("Cancel") {
                    config.settings = config.binding.as_ref().unwrap().clone().1;
                    config.binding = None;
                }
            }
            if ui.button("New") {
                open_popup();
            }
        });
}
