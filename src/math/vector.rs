use bevy::prelude::*;
use bevy::math::{DQuat, DVec3};

use std::fmt::Debug;
use std::ops::{Neg, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
//*
use fixed::types::{I48F16, I2F62, FixedI128, extra::U62};
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
//	Vector Macro
macro_rules! define_fixed_vec3 {
	($Name:ident, $Scalar:ty) => { 
		#[derive(
			Clone, Copy, Debug, Default, PartialEq,
			Reflect, Neg,
			Add, AddAssign, Sub, SubAssign,
			Mul, MulAssign, Div, DivAssign,
		)]
		#[mul(forward)] // Allows Vec * Scalar
		#[div(forward)] // Allows Vec / Scalar
		pub struct $Name {
			pub x: $Scalar,
			pub y: $Scalar,
			pub z: $Scalar,
		}

		impl $Name {
			//	Constructors
			pub const ZERO: Self = Self { x: <$Scalar>::ZERO, y: <$Scalar>::ZERO, z: <$Scalar>::ZERO }
			#[inline] fn new(x: <$Scalar>, y: <$Scalar>, z: <$Scalar>) -> Self { Self{ x, y, z } }

			//	Type conversion
			#[inline] fn to<V>(self) -> V where 
				V: TypeVec3, V::Scalar: fixed::traits::FromFixed, 
			{	V::new(
					V::Scalar::from_num(self.x),
					V::Scalar::from_num(self.y),
					V::Scalar::from_num(self.z),
			)	}
			#[inline] fn to_f64(self) -> DVec3 {
				DVec3::new(self.x.to_num(), self.y.to_num(), self.z.to_num())
			}

			//	Algebraic ops
			#[inline] fn dot(self, other: Self) -> $Scalar {
				self.x * other.x + self.y * other.y + self.z * other.z
			}
			#[inline] fn cross(self, other: Self) -> Self { Self {
					x: self.y * other.z - self.z * other.y,
					y: self.z * other.x - self.x * other.z,
					z: self.x * other.y - self.y * other.x,
			}	}
			#[inline] fn mag2(self) -> $Scalar { self.dot(self) }
		}

		impl TypeVec3 for $Name {
			type Scalar = $Scalar;

			fn new(x: $Scalar, y: $Scalar, z: $Scalar) -> Self { Self::new(x, y, z) }
			fn zero() -> Self { Self::ZERO }

			fn to_f64(self) -> DVec3 { self.to_f64() }

			fn dot(self, other: Self) -> $Scalar { self.dot(other) }
			fn cross(self, other: Self) -> Self { self.cross(other) }
			fn mag_sq(self) -> $Scalar { self.dot(self) }

			fn rotate(self, rot: DQuat) -> Self { todo!("rotate() implemented manually for specific types") }
		}
	};
}

//	Define types
pub type FixOrigin = I48F16;
pub type FixAngles = I2F62;

pub type FixWide = FixedI128<U62>;

define_fixed_vec3!(FixVec3, FixOrigin);
define_fixed_vec3!(FixAng3, FixAngles);

define_fixed_vec3!(FixWide3, FixWide);

//	Rotation
macro_rules! mul_mix { ($pos:expr, $ang:expr) => {{
	let p_bits = $pos.to_bits() as i128;
	let a_bits = $ang.to_bits() as i128;
	let r_bits = (p_bits * a_bits) >> 62;
	FixOrigin::from_bits(r_bits as i64)
}};	}

impl TypeVec3 for FixVec3 {
	type Scalar = FixOrigin;

	fn rotate(self, rot: DQuat) -> Self {
		//	Upcast for multiplication
		let v_wide = FixWide3::new(
            FixWide::from_num(self.x),
            FixWide::from_num(self.y),
            FixWide::from_num(self.z),
        );

		let qW_wide = FixWide::from_num(rot.w);
        let qXYZ_wide = FixWide3::new(
            FixWide::from_num(rot.x),
            FixWide::from_num(rot.y),
            FixWide::from_num(rot.z),
        );

		//	Math
		let a = qXYZ_wide.cross(v_wide) + (v_wide * qW_wide);
		let b = qXYZ_wide.cross(a);

		let res_wide = v_wide + (term_b * FixWide::from_num(2));
		res_wide.to()
	}
}