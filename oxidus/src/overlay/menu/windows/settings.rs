use crate::settings::{Entry, EntryValue, Settings};

pub fn show_settings(ui: &mut imgui::Ui) {
    ui.window("Settings").build(|| {
        let settings = Settings::get();
        let mut settings = settings.write().unwrap();
        let Entry::Group(ref mut aimbot) = settings.entries.get_mut("aimbot").unwrap() else {
            panic!("Invalid entry")
        };

        let Entry::Value(EntryValue::F32(ref mut fov),..) = aimbot.get_mut("fov").unwrap() else { panic!("Invalid entry") };
        ui.slider_config("fov", 0.0, 180.0).build(fov);
    });
}
