use nalgebra::{Vector2, Vector3, Vector4};
use num::complex::Complex32;

use crate::Type;

#[derive(Clone, Copy)]
pub enum Value {
    Complex(Complex32),
    Real(f32),
    Int(i32),
    Vec2(Vector2<f32>),
    Vec3(Vector3<f32>),
    Vec4(Vector4<f32>),
}

impl Value {
    pub fn ty(self) -> Type {
        match self {
            Value::Complex(_) => Type::Complex,
            Value::Real(_) => Type::Real,
            Value::Int(_) => Type::Int,
            Value::Vec2(_) => Type::Vec2,
            Value::Vec3(_) => Type::Vec3,
            Value::Vec4(_) => Type::Vec4,
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

    pub fn complex(self) -> Complex32 {
        let Self::Complex(v) = self else {
            unreachable!()
        };

        v
    }

    pub fn vec2(&self) -> Vector2<f32> {
        let &Self::Vec2(v) = self else { unreachable!() };

        v
    }

    pub fn vec3(&self) -> Vector3<f32> {
        let &Self::Vec3(v) = self else { unreachable!() };

        v
    }

    pub fn vec4(&self) -> Vector4<f32> {
        let &Self::Vec4(v) = self else { unreachable!() };

        v
    }
}
