use crate::settings::Settings;

pub fn show_config(ui: &mut imgui::Ui) {
    ui.window("Config file").build(|| {
        let settings = Settings::get();
        let mut settings = settings.write().unwrap();
        let path = settings.file_path.clone();
        if ui.button("Save") {
            settings.save().unwrap();
        }
        if ui.button("Load") {
            settings.load().unwrap();
        }

    });
}
