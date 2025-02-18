pub mod esp;
pub mod aimbot;

pub fn init_modules() {
    esp::init();
    aimbot::init();
}
