#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vector3 {
    pub fn dot(&self, other: &Vector3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Angles {
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VMatrixRow {
    pub vec: Vector3,
    pub w: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VMatrix {
    pub right: VMatrixRow,
    pub up: VMatrixRow,
    pub forward: VMatrixRow,
    pub origin: VMatrixRow,
}

impl VMatrix {
    pub fn transform_vector(&self, vec: &Vector3) -> (Vector3, f32) {
        (
            Vector3 {
                x: self.right.vec.dot(vec) + self.right.w,
                y: self.up.vec.dot(vec) + self.up.w,
                z: self.forward.vec.dot(vec) + self.forward.w,
            },
            self.origin.vec.dot(vec) + self.origin.w,
        )
    }
}
