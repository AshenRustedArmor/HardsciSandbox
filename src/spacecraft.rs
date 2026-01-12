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
#[derive(Debug)]
struct CraftOrigin {
	//	Here goes world coordinates, etc.

}

#[derive(Debug)]
struct CraftCompartment {
	//	CraftOrigin and craft-local position

	//
}