use imgui::{Ui, WindowFlags};
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use crate::i;
use crate::math::{Angles, Vector3};

// Debug state structure
pub struct DebugState {
    pub variables: HashMap<(&'static str, u32, String, Option<String>), String>,
    pub inputs: HashMap<(&'static str, u32, String), DebugInput>,
    pub points: HashMap<(&'static str, u32, String), Vector3>,
    pub angle: HashMap<(&'static str, u32, String), (Vector3, Angles)>,
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
                points: HashMap::new(),
                angle: HashMap::new(),
            })
        })
    }
}

/// Displays the debug window with all tracked variables and inputs
pub fn show_debug_window(ui: &mut Ui, visible: bool) {
    let mut state = DebugState::global().lock().unwrap();

    let bg_alpha = if visible { 1.0 } else { 0.2 };
    let flags = if visible {
        WindowFlags::empty()
    } else {
        WindowFlags::NO_TITLE_BAR | WindowFlags::NO_NAV | WindowFlags::NO_SCROLLBAR
    };

    ui.window("Debug")
        .size([300.0, 200.0], imgui::Condition::FirstUseEver)
        .collapsible(true)
        .flags(flags)
        .bg_alpha(bg_alpha)
        .resizable(visible)
        .movable(visible)
        .build(|| {
            //ui.child_window("variables")
            //    .bg_alpha(bg_alpha)
            //    .always_auto_resize(true)
            //    .flags(flags)
            //    .build(|| {
            for ((file, line, name, key), value) in &state.variables {
                if let Some(key) = key {
                    ui.text_wrapped(format!("{file}:{line}#{key} - {name} = {value}"));
                } else {
                    ui.text_wrapped(format!("{file}:{line} - {name} = {value}"));
                }
            }
            //    });

            //make a child window
            //ui.child_window("inputs")
            //    .bg_alpha(bg_alpha)
            //    .always_auto_resize(true)
            //    .flags(flags)
            //    .build(|| {
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
            //    });

            // Display and manage inputs

            if i!().engine.is_in_game() {
                let w2s = i!().client.get_w2s_matrix();
                let draw_list = ui.get_background_draw_list();
                let viewport = unsafe { imgui::sys::igGetMainViewport().read() };
                let window_size = [viewport.Size.x, viewport.Size.y];
                let scale = (window_size[0] as f32 / 2f32, window_size[1] as f32 / 2f32);
                for ((_, _, label), point3d) in &mut state.points {
                    let Some(mut point) = w2s.transform_vector(point3d) else {
                        continue;
                    };
                    point[0] *= scale.0;
                    point[1] *= scale.1;

                    draw_list
                        .add_circle(point, 1.0, 0xFF_FF_FF_FF)
                        .filled(true)
                        .build();
                    draw_list.add_text(point, 0xFF_FF_FF_FF, label);
                }
                for ((_, _, label), (point3d, angle)) in &mut state.angle {
                    let forward = angle.forward();
                    let point3d2 = *point3d + forward * 10.0;
                    let Some(mut point) = w2s.transform_vector(point3d) else {
                        continue;
                    };
                    point[0] *= scale.0;
                    point[1] *= scale.1;

                    let Some(mut point2) = w2s.transform_vector(&point3d2) else {
                        continue;
                    };
                    point2[0] *= scale.0;
                    point2[1] *= scale.1;

                    draw_list.add_line(point, point2, 0xFF_FF_FF_FF).build();
                    draw_list.add_text(point, 0xFF_FF_FF_FF, label);
                }
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
        state
            .variables
            .insert((file, line, name.into(), None), value);
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
            .insert((file, line, name.into(), Some($key.into())), value);
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
            .entry((file, line, label_str.into()))
            .or_insert($default.into());
        entry.clone().into()
    }};
}

#[macro_export]
macro_rules! mdbg_point {
    ($label: expr, $point:expr) => {{
        let file = file!();
        let line = line!();
        let mut state = $crate::overlay::menu::windows::debug::DebugState::global()
            .lock()
            .unwrap();
        state.points.insert((file, line, $label.into()), $point);
    }};
}

#[macro_export]
macro_rules! mdbg_angle {
    ($label: expr, $point:expr, $angle: expr) => {{
        let file = file!();
        let line = line!();
        let mut state = $crate::overlay::menu::windows::debug::DebugState::global()
            .lock()
            .unwrap();
        state
            .angle
            .insert((file, line, $label.into()), ($point, $angle));
    }};
}
