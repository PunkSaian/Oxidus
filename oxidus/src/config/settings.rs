use macros::settings;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsField<T> {
    value: T,
    default: T,
    #[serde(skip)]
    pub overwrite: Option<T>,
}

impl<T: Clone> SettingsField<T> {
    pub fn new(default: T) -> Self {
        Self {
            value: default.clone(),
            default,
            overwrite: None,
        }
    }

    pub fn get(&self) -> &T {
        self.overwrite.as_ref().unwrap_or(&self.value)
    }
    pub fn get_mut(&mut self) -> Option<&mut T> {
        if self.overwrite.is_some() {
            None
        } else {
            Some(&mut self.value)
        }
    }
}

impl<T: PartialEq + Clone> SettingsField<T> {
    pub fn merge(&mut self, old: &Self) {
        if self.default == old.default {
            self.value = old.value.clone();
        }
        if old.overwrite.is_some() {
            self.overwrite.clone_from(&old.overwrite);
        }
    }
    pub fn apply_overwrite(&mut self, bind: &Self) {
        if let Some(overwrite) = bind.overwrite.as_ref() {
            self.overwrite = Some(overwrite.clone());
        }
    }
    pub fn apply_overwrite_permanent(&mut self, bind: &Self) {
        if let Some(overwrite) = bind.overwrite.as_ref() {
            self.value = overwrite.clone();
        }
    }
    pub fn clear_overwrites(&mut self) {
        self.overwrite = None;
    }
    pub fn generate_overwrites(&mut self, new: &Self) {
        if self.value != new.value {
            self.overwrite = Some(new.value.clone());
        }
    }
}

settings!(
    aimbot {
        enabled: bool, false,
        fov: f32, 30.0,
        draw_fov: bool, false,
    }
    esp {
        enabled: bool, false
    }
    visual {
        fov: f32, 100.0,
        third_person: bool, false
    }
    movement {
        bhop: bool, false,
        momentum_compensation: bool, false,
        auto_strafe: bool, false
    }
);
