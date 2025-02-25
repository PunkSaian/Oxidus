use crate::math::Vector3;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Model {
    pub handle: &'static (),
    pub name: *const i8,
    pub load_flags: i32,
    pub server_count: i32,
    pub r#type: i32,
    pub flags: i32,
    pub vec_mins: Vector3,
    pub vec_maxs: Vector3,
    pub radius: f32,
}
