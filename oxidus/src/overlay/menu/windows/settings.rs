use std::{sync::OnceLock, thread};

use imgui::WindowFlags;

use crate::{
    oxidus_cleanup,
    settings::{Entry, EntryValue, Settings},
};

pub fn show_settings(ui: &mut imgui::Ui) {
    ui.window("Settings").menu_bar(true).flags(WindowFlags::NO_DOCKING| WindowFlags::MENU_BAR | WindowFlags::NO_COLLAPSE).build(|| {

        let settings = Settings::get();
        let mut settings = settings.write().unwrap();
        ui.modal_popup_config("new config").build(|| {
            static mut new_config_name: String = const { String::new() };

            ui.input_text("name", unsafe{&mut new_config_name}).build();
            static mut copy_current: bool = const { false };

            ui.checkbox("copy current", unsafe{&mut copy_current});
            ui.spacing();
            unsafe {
                if ui.button("create") && !new_config_name.is_empty(){
                    settings.create_new(&new_config_name, copy_current);
                    new_config_name.clear();
                    copy_current = false;
                    ui.close_current_popup();
                }
                ui.same_line();
                if ui.button("cancel") {
                    new_config_name.clear();
                    copy_current = false;
                    ui.close_current_popup();
                }
            }
        });
        let popup_id = ui.new_id_str("new config");
        ui.menu_bar(||{
            ui.menu("oxide", || {
            if ui.menu_item("unload") {
                thread::spawn(|| {
                    oxidus_cleanup();
                });
            }
            });
            ui.menu("config", || {
                if ui.menu_item("new") {
                    unsafe{
                        imgui::sys::igOpenPopup_ID(std::mem::transmute(popup_id), 0);
                    }
                }
                ui.separator();
                let current_config = settings.meta.current_config.clone();
                for entry in Settings::get_config_files().unwrap() {
                    let mut config_name = entry.file_stem().unwrap().to_str().unwrap().to_owned();

                    if current_config == entry{
                        config_name.push_str(" *");
                    }
                    ui.menu(&config_name, ||{
                        if ui.menu_item("select") {
                            settings.switch_config(&entry).unwrap();
                            settings.meta.current_config.clone_from(&entry);
                        }
                        if ui.menu_item_config("delete").enabled(settings.meta.current_config != entry).build(){
                            settings.delete_config(&entry).unwrap();
                        }
                    });
                    //if ui.menu_item_config().selected(current_config == entry).build() {
                    //}
                }
            });
        });

        let Entry::Group(ref mut aimbot) = settings.entries.get_mut("aimbot").unwrap() else {
            panic!("Invalid entry")
        };

        let Entry::Value(EntryValue::F32(ref mut fov),..) = aimbot.get_mut("fov").unwrap() else { panic!("Invalid entry") };
        ui.slider_config("fov", 0.0, 180.0).build(fov);
    });
}
