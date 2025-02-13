pub mod detour;
pub mod vmt;

pub fn restore_hooks() {
    detour::restore_detour_hooks();
    vmt::restore_vmt_hooks();
}
