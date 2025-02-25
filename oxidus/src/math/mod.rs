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
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3 { x, y, z }
    }
    pub fn empty() -> Self {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
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
    pub fn corners(&self, mins: &Vector3, maxs: &Vector3) -> [Vector3; 8] {
        *(0..8)
            .map(|i| {
                let mut pos = *self;
                if i & 1 == 0 {
                    pos.x += mins.x;
                } else {
                    pos.x += maxs.x;
                }
                if i & 2 == 0 {
                    pos.y += mins.y;
                } else {
                    pos.y += maxs.y;
                }
                if i & 4 == 0 {
                    pos.z += mins.z;
                } else {
                    pos.z += maxs.z;
                }
                pos
            })
            .collect::<Vec<_>>()
            .as_array()
            .unwrap()
    }
    pub fn rotate(&self, rotation: &RotationVectors) -> Vector3 {
        Vector3 {
            x: self.dot(&rotation.forward),
            y: self.dot(&rotation.right),
            z: self.dot(&rotation.up),
        }
    }
    pub fn normalized(&self) -> Option<Vector3> {
        let len = self.len();
        if len == 0.0 {
            return None;
        }
        Some(*self / len)
    }
}

impl std::ops::Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(mut self, rhs: f32) -> Self::Output {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
        self
    }
}

impl std::ops::Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(mut self, rhs: f32) -> Self::Output {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self
    }
}
impl std::ops::MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
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

impl std::ops::AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
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

#[derive(Debug, Clone, Copy)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl std::ops::Mul<Vector2> for Vector2 {
    type Output = Vector2;

    fn mul(mut self, rhs: Vector2) -> Self::Output {
        self.x *= rhs.x; 
        self.y *= rhs.y;
        self
    }
}

impl std::ops::MulAssign<Vector2> for Vector2 {
    fn mul_assign(&mut self, rhs: Vector2) {
        *self = *self * rhs;
    }
}

impl std::ops::Mul<f32> for Vector2 {
    type Output = Vector2;

    fn mul(mut self, rhs: f32) -> Self::Output {
        self.x *= rhs;
        self.y *= rhs;
        self
    }
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

impl Angles {
    pub fn forward(&self) -> Vector3 {
        let p = -self.pitch.to_radians();
        let y = self.yaw.to_radians() + 2.0 * std::f32::consts::PI;
        Vector3 {
            x: (p.cos() * y.cos()),
            y: (p.cos() * y.sin()),
            z: p.sin(),
        }
    }
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
    pub fn transform_vector(&self, vec: &Vector3) -> Option<[f32; 2]> {
        let x = self.right.vec.dot(vec) + self.right.w;
        let y = self.up.vec.dot(vec) + self.up.w;
        let w = self.origin.vec.dot(vec) + self.origin.w;

        if w < 0.01 {
            return None;
        }

        let x = 1.0 + (x / w);
        let y = 1.0 - (y / w);
        Some([x, y])
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RotationVectors {
    pub forward: Vector3,
    pub right: Vector3,
    pub up: Vector3,
}
