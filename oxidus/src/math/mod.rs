#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<[f32; 3]> for Vector3 {
    fn from(array: [f32; 3]) -> Self {
        Vector3 {
            x: array[0],
            y: array[1],
            z: array[2],
        }
    }
}

impl Vector3 {
    pub fn dot(&self, other: &Vector3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn squared_len(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn squared_len_2d(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }
    pub fn len_2d(&self) -> f32 {
        self.squared_len_2d().sqrt()
    }
    pub fn len(&self) -> f32 {
        self.squared_len().sqrt()
    }

    pub fn angle(&self) -> Angles {
        Angles {
            pitch: -self.z.atan2(self.len_2d()) / std::f32::consts::PI * 180f32,
            yaw: -(self.x.atan2(self.y) / std::f32::consts::PI * 180f32) + 90.0,
            roll: 0.0,
        }
    }
}

impl std::ops::Add for Vector3 {
    type Output = Vector3;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self
    }
}

impl std::ops::Sub for Vector3 {
    type Output = Vector3;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self
    }
}

pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl From<[f32; 2]> for Vector2 {
    fn from(array: [f32; 2]) -> Self {
        Vector2 {
            x: array[0],
            y: array[1],
        }
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
