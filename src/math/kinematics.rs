use bevy::prelude::*;
use bevy::math::{DVec3, DQuat};

use num_traits::NumCast;
use crate::math::primitives::TypeVec3;
use std::ops::{Add, AddAssign, Mul, MulAssign};

//		Screw theoretic primitives
//	Pose - position and orientation
#[derive(Clone, Copy, Debug, Component, Reflect)]
#[reflect(Component)]
pub struct Pose<V: Vector3> {
    pub origin: V,
    pub angles: DQuat,
}

//	Twist - linear & angular velocity
#[derive(Clone, Copy, Debug, Component, Reflect)]
#[reflect(Component)]
pub struct Pose<V: Vector3> {
    pub lin: V,
    pub ang: DVec3,
}