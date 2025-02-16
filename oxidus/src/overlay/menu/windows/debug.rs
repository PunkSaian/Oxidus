use imgui::{InputTextFlags, Ui, WindowFlags};
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

// Debug state structure
pub struct DebugState {
    pub variables: HashMap<(&'static str, u32, &'static str), String>,
    inputs: HashMap<(&'static str, u32, &'static str), String>,
    counters: HashMap<(&'static str, u32), usize>,
}

impl DebugState {
    pub fn global() -> &'static Mutex<Self> {
        static INSTANCE: OnceLock<Mutex<DebugState>> = OnceLock::new();
        INSTANCE.get_or_init(|| {
            Mutex::new(DebugState {
                variables: HashMap::new(),
                inputs: HashMap::new(),
                counters: HashMap::new(),
            })
        })
    }

    fn get_counter(&mut self, file: &'static str, line: u32) -> usize {
        let counter = self.counters.entry((file, line)).or_insert(0);
        *counter += 1;
        *counter
    }
}

/// Displays the debug window with all tracked variables and inputs
pub fn show_debug_window(ui: &mut Ui, visible: bool) {
    let mut state = DebugState::global().lock().unwrap();

    ui.window("Debug")
        .size([300.0, 200.0], imgui::Condition::FirstUseEver)
        .collapsible(true)
        .flags(WindowFlags::NO_TITLE_BAR)
        .bg_alpha(if visible { 1.0 } else { 0.2 })
        .resizable(visible)
        .movable(visible)
        .build(|| {
            // Display tracked variables
            for ((file, line, name), value) in &state.variables {
                ui.text_wrapped(format!("{file}:{line} - {name} = {value}"));
            }

            // Display and manage inputs
            for ((file, line, label), value) in &mut state.inputs {
                let mut buffer = value.clone();
                if ui
                    .input_text(&format!("{label}##{file}:{line}"), &mut buffer)
                    .flags(InputTextFlags::ENTER_RETURNS_TRUE)
                    .build()
                {
                    *value = buffer;
                }
            }
        });

    // Clear variables for next frame but keep inputs
    state.variables.clear();
}

/// Tracks a variable's value in the debug window
#[macro_export]
macro_rules! mdbg {
    ($var:expr) => {{
        let value = format!("{:?}", $var);
        let name = stringify!($var);
        let file = file!();
        let line = line!();
        let mut state = $crate::overlay::menu::windows::debug::DebugState::global()
            .lock()
            .unwrap();
        state.variables.insert((file, line, name), value);
    }};
}

/// Creates or updates an input field in the debug window
#[macro_export]
macro_rules! mdbg_input {
    ($label:expr) => {{
        let label_str = $label;
        let file = file!();
        let line = line!();
        let mut state = $crate::overlay::menu::windows::debug::DebugState::global()
            .lock()
            .unwrap();
        let entry = state
            .inputs
            .entry((file, line, label_str))
            .or_insert(String::new());
        entry.clone()
    }};
}

/// Tracks a variable with a counter for multiple entries on the same line
#[macro_export]
macro_rules! mdbg_counted {
    ($var:expr) => {{
        let value = format!("{:?}", $var);
        let name = stringify!($var);
        let file = file!();
        let line = line!();
        let mut state = $crate::overlay::menu::windows::debug::DebugState::global()
            .lock()
            .unwrap();
        let counter = state.get_counter(file, line);
        let label = format!("{} #{}", name, counter);
        state.variables.insert((file, line, &label), value);
    }};
}
