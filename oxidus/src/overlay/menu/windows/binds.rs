use crate::config::Config;
use imgui::{Key, WindowFlags};

#[allow(static_mut_refs, clippy::too_many_lines)]
pub fn show_binds(ui: &mut imgui::Ui) {
    pub static mut WAITING_FOR_KEYS: bool = false;
    pub static mut PRESED_KEYS: Vec<Key> = Vec::new();
    pub static mut NAME: String = String::new();

    let mut config = Config::get_write();

    // Create new bind popup
    ui.modal_popup_config("new bind")
        .resizable(false)
        .movable(false)
        .always_auto_resize(true)
        .build(|| unsafe {
            // ... [same as original] ...
        });

    // Edit bind window
    let popup_id = ui.new_id_str("new bind");
    let open_popup = || unsafe { /* ... [same as original] ... */
    };

    ui.window("Binds")
        .flags(WindowFlags::NO_DOCKING | WindowFlags::NO_COLLAPSE | WindowFlags::ALWAYS_AUTO_RESIZE)
        .build(|| {
            // Bind list table
            if let Some(table) = ui.begin_table("binds", 3) {
                // ... [same table setup as original] ...

                for (i, bind) in config.binds.binds.clone().iter().enumerate() {
                    // ... [same row rendering as original] ...

                    ui.table_next_column();
                    ui.disabled(config.binding.is_some(), || {
                        //if ui.button(format!("Edit##{i}")) {
                        //    config.binding = Some((i, config.settings_old.clone()));
                        //}
                        // ... [same delete button] ...
                    });
                }
                table.end();
            }

            // Settings override panel
            if let Some((bind_idx, original_settings)) = &config.binding {
                //let bind = &mut config.binds.binds[*bind_idx];

                // Settings diff table
                if let Some(table) = ui.begin_table("settings", 3) {
                    ui.table_setup_column("Path");
                    ui.table_setup_column("Value");
                    ui.table_setup_column("Options");

                    //TODO(oxy): implement diff_settings
                    //for (path, value) in &bind.overwrites {
                    //    ui.table_next_row();
                    //    ui.table_next_column();
                    //    ui.text(&path.join(" > "));

                    //    ui.table_next_column();
                    //    ui.text(format!("{value:?}"));

                    //    ui.table_next_column();
                    //    if ui.button(format!("Delete##{}", path.join("-"))) {
                    //        bind.overwrites.remove(path);
                    //    }
                    //}
                    table.end();
                }

                // Save/Cancel buttons
                if ui.button("Save") {
                    //let diff = diff_settings(&original_settings, &config.settings_old);
                    //bind.overwrites = diff;
                    //config.binding = None;
                }

                ui.same_line();
                if ui.button("Cancel") {
                    //config.settings_old = original_settings.clone();
                    //config.binding = None;
                }
            }

            if ui.button("New") {
                open_popup();
            }
        });
}
