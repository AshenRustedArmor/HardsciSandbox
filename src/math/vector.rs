use bevy::prelude::*;
use bevy::math::{DQuat, DVec3};

use std::fmt::Debug;
use std::ops::{Neg, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
//*
use fixed::{FixedI128, traits::FromFixed};
use fixed::types::{I48F16, I2F62, extra::U62};
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

//		Fixed-point math
//	Vector Macro
macro_rules! define_fixed_vec3 {
	//	1)	public-facing macro
	//	Creates *all* vector ops, even those between other types
	($Name:ident, $Scalar:ty, $Wide:ty) => { paste::paste!{
			//	Internal struct defs
			define_fixed_vec3!(@internal_struct [<$Name Wide>], $Wide)
			define_fixed_vec3!(@internal_struct $Name, $Scalar)

			impl TypeVec3 for $Name {
				type Scalar = $Scalar;

				//	Constructors
				#[inline] fn new(x: $Scalar, y: $Scalar, z: $Scalar) -> Self { Self{ x, y, z } }
				#[inline] fn zero() -> Self { Self::ZERO }
				
				//	Access/conversion
				#[inline] fn x(&self) -> $Scalar { self.x }
                #[inline] fn y(&self) -> $Scalar { self.y }
                #[inline] fn z(&self) -> $Scalar { self.z }
				
				#[inline] fn to_f64(self) -> DVec3 { self.to_f64() 
					DVec3::new(self.x.to_num(), self.y.to_num(), self.z.to_num())
				}

				//	Arithmetic ops
				#[inline] fn dot(self, other: Self) -> $Scalar { self.dot(other) }
				#[inline] fn cross(self, other: Self) -> Self { self.cross(other) }
				#[inline] fn mag2(self) -> $Scalar { self.dot(self) }

				//	Geometric ops
				fn rotate(self, rot: DQuat) -> Self {
					//	Upconversion
					let v_xyz = [<$Name Wide>] = self.to(); //	Handy hack from implementing .to()

					let q_w = <$Wide>::from_num(rot.w);
					let q_xyz = [<$Name Wide>]::new(
						<$Wide>::from_num(rot.x),
						<$Wide>::from_num(rot.y),
						<$Wide>::from_num(rot.z),
					);

					//	Math:	q X (q X v + v*w)
					let t1 = q_xyz.cross(v_xyz) + (v_xyz * q_w);
					let res = v_xyz + (q_xyz.cross(t1) * <$Wide>::from_num(2))
					res.to() //	Handy hack from implementing .to()
				}
			}

			impl TypeVec3 for [<$Name Wide>] {
				type Scalar = $Wide;

				//	Constructors
				#[inline] fn new(x: $Wide, y: $Wide, z: $Wide) -> Self { Self::new(x, y, z) }
				#[inline] fn zero() -> Self { Self::ZERO }

				//	Access/conversion
				#[inline] fn x(&self) -> $Wide { self.x }
                #[inline] fn y(&self) -> $Wide { self.y }
                #[inline] fn z(&self) -> $Wide { self.z }

				#[inline] fn to_f64(self) -> DVec3 { self.to_f64() }

				//	Arithmetic ops
				#[inline] fn dot(self, other: Self) -> $Wide { self.dot(other) }
				#[inline] fn cross(self, other: Self) -> Self { self.cross(other) }
				#[inline] fn mag2(self) -> $Wide { self.dot(self) }

				//	Geometric ops
				#[inline] fn rotate(self, _: DQuat) -> Self { panic!("DQuat rotation supported only by exposed vector type.") }
			}
	}	};

	//	2)	boilerplate: internal struct generator
	//	Creates fixed-point specific vector ops
	(@internal_struct $Name:ident, $Scalar:ty) => {
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
			#[inline(always)] pub fn new(x: <$Scalar>, y: <$Scalar>, z: <$Scalar>) -> Self { Self{ x, y, z } }

			//	Access/conversion
			#[inline] pub fn to<V>(self) -> V where 
				V: TypeVec3, V::Scalar: FromFixed, 
			{	V::new(
					V::Scalar::from_num(self.x),
					V::Scalar::from_num(self.y),
					V::Scalar::from_num(self.z),
			)	}
			#[inline] pub fn to_f64(self) -> DVec3 {
				DVec3::new(self.x.to_num(), self.y.to_num(), self.z.to_num())
			}

			//	Algebraic ops
			#[inline] pub fn dot(self, other: Self) -> $Scalar {
				self.x * other.x + self.y * other.y + self.z * other.z
			}
			#[inline] pub fn cross(self, other: Self) -> Self { Self {
					x: self.y * other.z - self.z * other.y,
					y: self.z * other.x - self.x * other.z,
					z: self.x * other.y - self.y * other.x,
			}	}
		}
	};
}

//	Define types
pub type FixOrigin = I48F16;
//pub type FixAngles = I2F62;
pub type FixWide = FixedI128<U62>;

define_fixed_vec3!(FixVec3, FixOrigin, FixWide);
//define_fixed_vec3!(FixAng3, FixAngles);

//	Implement TypeVec3 for Bevy's DVec3
impl TypeVec3 for DVec3 {
	type Scalar = f64;
	
	//	Constructors
	#[inline(always)] fn new(x: f64, y: f64, z: f64) -> Self { Self::new(x, y, z) }
	#[inline(always)] fn zero() -> Self { Self::ZERO }

	//	Access/conversions
	#[inline(always)] fn x(&self) -> f64 { self.x }
    #[inline(always)] fn y(&self) -> f64 { self.y }
    #[inline(always)] fn z(&self) -> f64 { self.z }

	#[inline(always)] fn to_f64(self) -> DVec3 { self }

	//	Arithmetic
	#[inline(always)] fn dot(self, other: Self) -> f64 { self.dot(other) }
	#[inline(always)] fn cross(self, other: Self) -> Self { self.cross(other) }
	#[inline(always)] fn mag2(self) -> f64 { self.mag2() }

	//	Geometry
	#[inline(always)] fn rotate(self, rot: DQuat) -> Self { rot * self }
}