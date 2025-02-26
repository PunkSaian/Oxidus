use std::thread;

use imgui::{Id, WindowFlags};
use macros::settings_field;

use crate::{config::Config, oxidus_cleanup};
#[allow(static_mut_refs, clippy::too_many_lines)]
pub fn show_settings(ui: &mut imgui::Ui) {
    let mut config = Config::get_write();
    ui.modal_popup_config("new config")
        .resizable(false)
        .movable(false)
        .always_auto_resize(true)
        .build(|| {
            static mut NEW_CONFIG_NAME: String = const { String::new() };
            static mut COPY_CURRENT: bool = const { false };

            ui.input_text("name", unsafe { &mut NEW_CONFIG_NAME })
                .build();

            ui.checkbox("copy current", unsafe { &mut COPY_CURRENT });
            ui.spacing();
            unsafe {
                if ui.button("create") && !NEW_CONFIG_NAME.is_empty() {
                    config.create(&NEW_CONFIG_NAME, COPY_CURRENT).unwrap();
                    NEW_CONFIG_NAME.clear();
                    COPY_CURRENT = false;
                    ui.close_current_popup();
                }
                ui.same_line();
                if ui.button("cancel") {
                    NEW_CONFIG_NAME.clear();
                    COPY_CURRENT = false;
                    ui.close_current_popup();
                }
            }
        });
    let popup_id = ui.new_id_str("new config");
    let title = if let Some((bind_index, _)) = config.binding {
        let bind = config.binds.binds.get(bind_index).unwrap();
        format!("Settings [binding \"{}\"]###Settings", bind.name)
    } else {
        "Settings###Settings".to_owned()
    };
    ui.window(&title)
        .menu_bar(true)
        .flags(
            WindowFlags::NO_DOCKING
                | WindowFlags::MENU_BAR
                | WindowFlags::NO_COLLAPSE
                | WindowFlags::NO_RESIZE
                | WindowFlags::ALWAYS_AUTO_RESIZE,
        )
        .build(|| {
            ui.menu_bar(|| {
                ui.menu("oxidus", || {
                    if ui.menu_item("unload") {
                        thread::spawn(|| {
                            oxidus_cleanup();
                        });
                    }
                });
                ui.menu("config", || {
                    if ui.menu_item("new") {
                        unsafe {
                            imgui::sys::igOpenPopup_ID(std::mem::transmute::<Id, u32>(popup_id), 0);
                        }
                    }
                    ui.separator();
                    let current_config = config.meta.current_config.clone();
                    for entry in Config::get_config_files().unwrap() {
                        let mut config_name =
                            entry.file_stem().unwrap().to_str().unwrap().to_owned();

                        if current_config == entry {
                            config_name.push_str(" *");
                        }
                        ui.menu(&config_name, || {
                            if ui.menu_item("select") {
                                config.switch_config(&entry).unwrap();
                                config.meta.current_config.clone_from(&entry);
                            }
                            if ui
                                .menu_item_config("delete")
                                .enabled(config.meta.current_config != entry)
                                .build()
                            {
                                Config::delete(&entry).unwrap();
                            }
                        });
                    }
                });
            });

            //aimbot
            {
                #[settings_field(config.settings.aimbot.enabled)]
                {
                    ui.checkbox("enabled", enabled);
                }
                #[settings_field(config.settings.aimbot.fov)]
                {
                    ui.input_float("fov", fov).step(1.0).build();
                    *fov = fov.clamp(1.0, 180.0);
                }
            }
            //visual
            {
                #[settings_field(config.settings.visual.fov)]
                {
                    ui.input_float("fov", fov).step(1.0).build();
                    *fov = fov.clamp(1.0, 180.0);
                }
                #[settings_field(config.settings.visual.third_person)]
                {
                    ui.checkbox("third person", third_person);
                }
            }
            //movement
            {
                #[settings_field(config.settings.movement.auto_strafe)]
                {
                    ui.checkbox("auto strafe", auto_strafe);
                }
                #[settings_field(config.settings.movement.momentum_compensation)]
                {
                    ui.checkbox("momentum compensation", momentum_compensation);
                }
            }
        });
}
