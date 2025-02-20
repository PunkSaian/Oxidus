use std::thread;

use imgui::WindowFlags;

use crate::{
    oxidus_cleanup,
    settings::{Entry, EntryValue, Settings},
};

pub fn show_settings(ui: &mut imgui::Ui) {
    ui.window("Settings").menu_bar(true).flags(WindowFlags::NO_DOCKING| WindowFlags::MENU_BAR | WindowFlags::NO_COLLAPSE).build(|| {

        let settings = Settings::get();
        let mut settings = settings.write().unwrap();
        ui.menu_bar(||{
            ui.menu("oxide", || {
            if ui.menu_item("unload") {
                thread::spawn(|| {
                    oxidus_cleanup();
                });
            }
            });
            ui.menu("config", || {
                if ui.menu_item("save") {
                    settings.save_config().unwrap();
                }
                ui.menu("select", || {
                    let current_config = settings.meta.current_config.clone();
                    for entry in Settings::get_config_files().unwrap() {
                        if ui.menu_item_config(entry.to_str().unwrap()).selected(current_config == entry).build() {
                            settings.switch_config(&entry).unwrap();
                            settings.meta.current_config = entry;
                        }
                    }
                });
            });
        });
        let Entry::Group(ref mut aimbot) = settings.entries.get_mut("aimbot").unwrap() else {
            panic!("Invalid entry")
        };

        let Entry::Value(EntryValue::F32(ref mut fov),..) = aimbot.get_mut("fov").unwrap() else { panic!("Invalid entry") };
        ui.slider_config("fov", 0.0, 180.0).build(fov);
    });
}
