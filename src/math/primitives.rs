use bevy::prelude::*;
use bevy::math::{DQuat, DVec3};

use std::fmt::Debug;
use std::ops::{Neg, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
//*
use fixed::types::I48F16;
use derive_more::{Neg, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
// */

//		Vector abstraction
//	Trait
pub trait TypeVec3:
	//	Rust traits
	Clone + Copy + Debug + Default + PartialEq
	+ Send + Sync + 'static
	//	Algebraic traits
	+ Reflect + Neg<Output = Self>
	+ Add<Output = Self> + AddAssign
	+ Sub<Output = Self> + SubAssign
	+ Mul<Self::Scalar, Output = Self> + MulAssign<Self::Scalar>
	+ Div<Self::Scalar, Output = Self> + DivAssign<Self::Scalar>
{
	type Scalar: Clone + Copy
		+ Num + NumCast 
		+ PartialEq + PartialOrd 
		+ Debug;

	//	Constructors
	fn new(x: Self::Scalar, y: Self::Scalar, z: Self::Scalar) -> Self;
	fn zero() -> Self;

	fn to_f64(self) -> DVec3;

	//	Components
	fn x(&self) -> Self::Scalar;
	fn y(&self) -> Self::Scalar;
	fn z(&self) -> Self::Scalar;

	//	Algebraic operations
	fn dot(self, other: Self) -> Self::Scalar;
	fn cross(self, other: Self) -> Self;
	fn mag2(self) -> Self::Scalar;

	//	Rotations
	fn rotate(self, rot: DQuat) -> Self;
}

//	Implement for Bevy's DVec3
impl TypeVec3 for DVec3 {
	type Scalar = f64;
	
	#[inline(always)] fn new(x: f64, y: f64, z: f64) -> Self { Self::new(x, y, z) }
	#[inline(always)] fn zero() -> Self { Self::ZERO }
	#[inline(always)] fn to_f64(self) -> DVec3 { self }

	#[inline(always)] fn dot(self, other: Self) -> f64 { self.dot(other) }
	#[inline(always)] fn cross(self, other: Self) -> Self { self.cross(other) }
	#[inline(always)] fn mag2(self) -> f64 { self.mag2() }

	#[inline(always)] fn rotate(self, rot: DQuat) -> Self { rot * self }
}

//		Fixed-point math
//	Define type
pub type FixOrigin = I48F16;
pub type FixAngles = I2F62;

#[derive(
	Clone, Copy, Debug, Default, PartialEq,
	Reflect, Neg,
	Add, AddAssign, Sub, SubAssign,
	Mul, MulAssign, Div, DivAssign,
)]
#[mul(forward)] // Allows FixVec3 * FixOrigin
#[div(forward)] // Allows FixVec3 / FixOrigin
pub struct FixVec3 {
	pub x: FixOrigin,
	pub y: FixOrigin,
	pub z: FixOrigin,
}

impl TypeVec3 for FixVec3 {
	type Scalar = FixOrigin;

	//	Constructors
	#[inline] fn new(x: FixOrigin, y: FixOrigin, z: FixOrigin) -> Self { Self{ x, y, z } }
	#[inline] fn zero() -> Self { Self{ 
			x: FixOrigin::ZERO, y: FixOrigin::ZERO, z: FixOrigin::ZERO
	} 	}
	#[inline] fn to_f64(self) -> DVec3 {
		DVec3::new(self.x.to_num(), self.y.to_num(), self.z.to_num())
	}

	//	Algebraic ops
	#[inline] fn dot(self, other: Self) -> FixOrigin {
		self.x * other.x + self.y * other.y + self.z * other.z
	}
	#[inline] fn cross(self, other: Self) -> Self { Self {
			x: self.y * other.z - self.z * other.y,
			y: self.z * other.x - self.x * other.z,
			z: self.x * other.y - self.y * other.x,
	}	}
	#[inline] fn mag2(self) -> FixOrigin { self.dot(self) }

	//	Geometry
	fn rotate(self, rot: DQuat) -> Self {
		let q_w = FixAngles::from_num(rot.w);
		let q_x = FixAngles::from_num(rot.x);
		let q_y = FixAngles::from_num(rot.y);
		let q_z = FixAngles::from_num(rot.z);

		//	Cross product
		let t_x = (q_y * self.z - q_z * self.y) + (q_w * self.x);
		let t_y = (q_z * self.x - q_x * self.z) + (q_w * self.y);
		let t_z = (q_x * self.y - q_y * self.x) + (q_w * self.z);

		//	Second cross
		let r_x = (q_y * t_z - q_z * t_y);
		let r_y = (q_z * t_x - q_x * t_z);
		let r_z = (q_x * t_y - q_y * t_x);

		//	Return
		Self {
			x: self.x + FixOrigin::from_num(r_x * 2),
			y: self.x + FixOrigin::from_num(r_y * 2),
			z: self.x + FixOrigin::from_num(r_z * 2),
		}
	}
}