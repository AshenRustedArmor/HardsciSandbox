//  Imports
use bevy::prelude::*;

use fixed::{
	types::I48F16,
	traits::ToFixed,
};

use std::ops::{
	Add, AddAssign,
	Sub, SubAssign,
	Mul, MulAssign,
	Div, DivAssign,
	Neg,
};

//	Objects
#[derive(Clone, Copy, Debug)]
struct OrbitVec {
	x: I48F16,
	y: I48F16,
	z: I48F16,
}

impl OrbitVec {
	//	Initialization
	fn new(x: I48F16, y: I48F16, z: I48F16) -> Self {
		Self { x, y, z }
	}
	
	fn zero() -> Self {
		Self { 
			
		}
	}


	fn local(&self, local: &OrbitVec) -> OrbitVec {
		OrbitVec::new(())
	}



	fn from_vec3(x: f64, y: f64, z: f64) -> Self { Self {
			x: I48F16::from_num(x),
			y: I48F16::from_num(y),
			z: I48F16::from_num(z),
	}	}
}


//  Component
#[derive(Component)]
struct OrbitPosition {

}


#[derive(Component)]
struct OrbitVelocity {

}

// #[derive(Component)]
// struct OrbitParams {
// 	axi: f32,					//	Semi-major axis
// 	ecc: f32,					//	Eccentricity

// 	inc: f32,					//	Inclination
// 	asc: f32,					//	Longitude of ascending node
// 	arg: f32,					//	Argument of periapsis

// 	anm: f32,					//	True anomaly
// }
