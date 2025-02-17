use imgui::{InputTextFlags, Ui, WindowFlags};
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

// Debug state structure
pub struct DebugState {
    pub variables: HashMap<(&'static str, u32, &'static str, Option<String>), String>,
    pub inputs: HashMap<(&'static str, u32, &'static str), DebugInput>,
}

#[derive(Clone)]
pub enum DebugInput {
    String(String),
    F32(f32),
    I32(i32),
}

impl From<String> for DebugInput {
    fn from(value: String) -> Self {
        DebugInput::String(value)
    }
}

impl From<f32> for DebugInput {
    fn from(value: f32) -> Self {
        DebugInput::F32(value)
    }
}

impl From<i32> for DebugInput {
    fn from(value: i32) -> Self {
        DebugInput::I32(value)
    }
}

impl From<DebugInput> for String {
    fn from(val: DebugInput) -> Self {
        match val {
            DebugInput::String(value) => value,
            DebugInput::F32(value) => value.to_string(),
            DebugInput::I32(value) => value.to_string(),
        }
    }
}

impl From<DebugInput> for f32 {
    fn from(val: DebugInput) -> Self {
        match val {
            DebugInput::String(value) => value.parse().unwrap_or(0.0),
            DebugInput::F32(value) => value,
            DebugInput::I32(value) => value as f32,
        }
    }
}

impl From<DebugInput> for i32 {
    fn from(val: DebugInput) -> Self {
        match val {
            DebugInput::String(value) => value.parse().unwrap_or(0),
            DebugInput::F32(value) => value as i32,
            DebugInput::I32(value) => value,
        }
    }
}

impl DebugState {
    pub fn global() -> &'static Mutex<Self> {
        static INSTANCE: OnceLock<Mutex<DebugState>> = OnceLock::new();
        INSTANCE.get_or_init(|| {
            Mutex::new(DebugState {
                variables: HashMap::new(),
                inputs: HashMap::new(),
            })
        })
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
            for ((file, line, name, key), value) in &state.variables {
                if let Some(key) = key {
                    ui.text_wrapped(format!("{file}:{line}#{key} - {name} = {value}"));
                } else {
                    ui.text_wrapped(format!("{file}:{line} - {name} = {value}"));
                }
            }

            // Display and manage inputs
            for ((file, line, label), value) in &mut state.inputs {
                match value {
                    DebugInput::F32(value) => ui
                        .input_float(&format!("{label}##{file}:{line}"), value)
                        .build(),
                    DebugInput::I32(ref mut value) => ui
                        .input_int(&format!("{label}##{file}:{line}"), value)
                        .build(),
                    DebugInput::String(ref mut value) => ui
                        .input_text(&format!("{label}##{file}:{line}"), value)
                        .build(),
                };
            }
        });
}

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
        state.variables.insert((file, line, name, None), value);
    }};
    ($key: expr, $var:expr) => {{
        let value = format!("{:?}", $var);
        let name = stringify!($var);
        let file = file!();
        let line = line!();
        let mut state = $crate::overlay::menu::windows::debug::DebugState::global()
            .lock()
            .unwrap();
        state
            .variables
            .insert((file, line, name, Some($key.into())), value);
    }};
}

#[macro_export]
macro_rules! mdbg_input {
    ($label:expr, $default:expr) => {{
        let label_str = $label;
        let file = file!();
        let line = line!();
        let mut state = $crate::overlay::menu::windows::debug::DebugState::global()
            .lock()
            .unwrap();
        let entry = state
            .inputs
            .entry((file, line, label_str))
            .or_insert($default.into());
        entry.clone().into()
    }};
}
