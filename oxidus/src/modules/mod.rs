pub mod aimbot;
pub mod esp;
pub mod movement;

pub fn init_modules() {
    esp::init();
    aimbot::init();
    movement::init();
}
