//  	Imports
use bevy::prelude::*;
use bevy::math::{DVec3, DQuat};

use fixed::types::I48F16;
use az::Cast;

use derive_more::{Neg, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

//		Definitions
pub type FixedType = I48F16;

//	World Frame
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug, Reflect)]
#[derive(Neg, Add, Sub, AddAssign, SubAssign)]
#[derive(Mul, Div, MullAssign, DivAssign)]
#[mul(forward)]
#[div(forward)]
pub struct WorldVec3 {
	pub x: FixedType,
	pub y: FixedType,
	pub z: FixedType
}

impl WorldVec3 {
	//	Constructors
	pub const ZERO: Self = Self { x: FixedType::ZERO, y: FixedType::ZERO, z: FixedType::ZERO };
	pub fn new<N: Cast<FixScalar>>(x: N, y: N, z: N) -> Self {
		Self { x: x.cast(), y: y.cast(), z: z.cast() }
	}

	//	Casting
}

#[derive(Clone, Copy, Debug, Reflect, Component)]
#[reflect(Component)]
pub struct WorldPose {
	pub pos: WorldVec3,
	pub rot: DQuat,
}

#[derive(Clone, Copy, Debug, Reflect, Component)]
#[reflect(Component)]
pub struct WorldTwist {
	pub lin: WorldVec3,
	pub ang: DVec3,
}

//	Local frame
#[derive(Clone, Copy, Debug, Reflect, Component)]
#[reflect(Component)]
pub struct ParentFrame(pub Entity);

#[derive(Clone, Copy, Debug, Reflect, Component)]
#[reflect(Component)]
pub struct LocalPose {
	pub pos: DVec3,
	pub rot: DQuat,
}

#[derive(Clone, Copy, Debug, Reflect, Component)]
#[reflect(Component)]
pub struct LocalTwist {
	pub lin: DVec3,
	pub ang: DVec3,
}

//	Implementations
impl WorldPose {
	
}