use std::ops::*;

#[macro_use]
mod macros;
mod vec2;
mod vec3;
mod vec4;

pub use vec2::Vec2;
pub use vec3::Vec3;
pub use vec4::Vec4;

impl_vec_ops!(Vec2, x, y);
