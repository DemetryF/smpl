use nalgebra::{Vector2, Vector3, Vector4};

use crate::Type;

#[derive(Clone, Copy)]
pub enum Value {
    Real(f32),
    Int(i32),
    F32x2(Vector2<f32>),
    F32x3(Vector3<f32>),
    F32x4(Vector4<f32>),
}

impl Value {
    pub fn ty(self) -> Type {
        match self {
            Value::Real(_) => Type::Real,
            Value::Int(_) => Type::Int,
            Value::F32x2(_) => Type::F32x2,
            Value::F32x3(_) => Type::F32x3,
            Value::F32x4(_) => Type::F32x4,
        }
    }

    pub fn int(self) -> i32 {
        let Self::Int(v) = self else { unreachable!() };

        v
    }

    pub fn real(self) -> f32 {
        let Self::Real(v) = self else { unreachable!() };

        v
    }

    pub fn f32x2(&self) -> Vector2<f32> {
        let &Self::F32x2(v) = self else {
            unreachable!()
        };

        v
    }

    pub fn f32x3(&self) -> Vector3<f32> {
        let &Self::F32x3(v) = self else {
            unreachable!()
        };

        v
    }

    pub fn f32x4(&self) -> Vector4<f32> {
        let &Self::F32x4(v) = self else {
            unreachable!()
        };

        v
    }
}
