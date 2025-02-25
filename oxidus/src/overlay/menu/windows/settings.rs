use std::thread;

use imgui::{Id, WindowFlags};

use crate::{config::Config, get_setting_mut, oxidus_cleanup};
#[allow(static_mut_refs, clippy::too_many_lines)]
pub fn show_settings(ui: &mut imgui::Ui) {
    let mut config = Config::get();
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
                    config.create_new(&NEW_CONFIG_NAME, COPY_CURRENT).unwrap();
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
                                Config::delete_config(&entry).unwrap();
                            }
                        });
                    }
                });
            });

            let enabled = get_setting_mut!(&mut config.settings, "aimbot", "enabled" => Bool);
            ui.checkbox("enabled", enabled);
            let fov = get_setting_mut!(&mut config.settings, "aimbot", "fov" => F32);
            ui.input_float("fov", fov).step(1.0).build();
            *fov = fov.clamp(1.0, 180.0);
            {
                let fov = get_setting_mut!(&mut config.settings, "visual", "fov" => F32);
                ui.input_float("visual fov", fov).step(1.0).build();
                *fov = fov.clamp(1.0, 180.0);
                let third_person =
                    get_setting_mut!(&mut config.settings, "visual", "third_person" => Bool);
                ui.checkbox("third person", third_person);
            }
            let momentum_compensation =
                get_setting_mut!(&mut config.settings, "movement", "momentum_compensation" => Bool);
            ui.checkbox("momentum compensation", momentum_compensation);
            let bhop = get_setting_mut!(&mut config.settings, "movement", "bhop" => Bool);
            ui.checkbox("bhop", bhop);
            let auto_strafe =
                get_setting_mut!(&mut config.settings, "movement", "auto_strafe" => Bool);
            ui.checkbox("auto strafe", auto_strafe);
        });
}
