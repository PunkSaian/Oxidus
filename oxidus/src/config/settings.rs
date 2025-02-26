use macros::settings;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsField<T> {
    pub value: T,
    pub default: T,
}

settings!(
    aimbot {
        enabled: bool, false,
        fov: f32, 30.0,
    }
    esp {
        enabled: bool, false
    }
    visual {
        fov: f32, 100.0,
        third_person: bool, false
    }
    movement {
        bhop: bool, true,
        momentum_compensation: bool, false,
        auto_strafe: bool, false
    }
);
