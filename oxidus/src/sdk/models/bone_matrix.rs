use crate::math::{RotationVectors, Vector3};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BoneMatrix(pub [[f32; 4]; 3]);

impl BoneMatrix {
    pub fn rotation(&self) -> RotationVectors {
        RotationVectors {
            forward: Vector3 {
                x: self.0[0][0],
                y: self.0[0][1],
                z: self.0[0][2],
            },
            right: Vector3 {
                x: self.0[1][0],
                y: self.0[1][1],
                z: self.0[1][2],
            },
            up: Vector3 {
                x: self.0[2][0],
                y: self.0[2][1],
                z: self.0[2][2],
            },
        }
    }
    pub fn position(&self) -> Vector3 {
        Vector3 {
            x: self.0[0][3],
            y: self.0[1][3],
            z: self.0[2][3],
        }
    }
}
