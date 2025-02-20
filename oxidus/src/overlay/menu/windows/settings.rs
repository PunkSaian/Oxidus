use std::thread;

use imgui::{Id, WindowFlags};

use crate::{get_entry_mut, oxidus_cleanup, settings::Settings};

#[allow(static_mut_refs)]
pub fn show_settings(ui: &mut imgui::Ui) {
    ui.window("Settings")
        .menu_bar(true)
        .flags(WindowFlags::NO_DOCKING | WindowFlags::MENU_BAR | WindowFlags::NO_COLLAPSE)
        .build(|| {
            let settings = Settings::get();
            let mut settings = settings.write().unwrap();
            ui.modal_popup_config("new config").build(|| {
                static mut NEW_CONFIG_NAME: String = const { String::new() };
                static mut COPY_CURRENT: bool = const { false };

                ui.input_text("name", unsafe { &mut NEW_CONFIG_NAME })
                    .build();

                ui.checkbox("copy current", unsafe { &mut COPY_CURRENT });
                ui.spacing();
                unsafe {
                    if ui.button("create") && !NEW_CONFIG_NAME.is_empty() {
                        settings.create_new(&NEW_CONFIG_NAME, COPY_CURRENT).unwrap();
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
                    let current_config = settings.meta.current_config.clone();
                    for entry in Settings::get_config_files().unwrap() {
                        let mut config_name =
                            entry.file_stem().unwrap().to_str().unwrap().to_owned();

                        if current_config == entry {
                            config_name.push_str(" *");
                        }
                        ui.menu(&config_name, || {
                            if ui.menu_item("select") {
                                settings.switch_config(&entry).unwrap();
                                settings.meta.current_config.clone_from(&entry);
                            }
                            if ui
                                .menu_item_config("delete")
                                .enabled(settings.meta.current_config != entry)
                                .build()
                            {
                                Settings::delete_config(&entry).unwrap();
                            }
                        });
                    }
                });
            });

            let fov = get_entry_mut!(&mut settings.config, "aimbot", "fov" => F32);
            ui.slider_config("fov", 0.0, 180.0).build(fov);
        });
}
