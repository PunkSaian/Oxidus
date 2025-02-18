pub mod aimbot;
pub mod esp;

pub fn init_modules() {
    esp::init();
    aimbot::init();
}
