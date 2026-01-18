//  	Imports
//	Bevy base
use bevy::prelude::*;
use bevy::math::{DVec3, DQuat};

//	Data handling
use fixed::types::I48F16;
pub type WorldFixed = I48F16;

//		Structures and components
//	"primitives"
pub struct FixVec3 {
	pub x: I48F16,
	pub y: I48F16,
	pub z: I48F16,
}

impl FixVec3{
	fn new(x: I48I16, y: I48F16, z: I48F16) -> Self { Self {x, y, z} }
}

//	Screw theoretic components
#[derive(Debug, Clone, Copy)]
pub struct Pose<P, R> {
	pub pos: P,
	pub rot: R,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Twist<L, A> {
	pub lin: L,
	pub ang: A,
}

//		ECS
//	Components
#[derive(Component, Debug, Clone, Copy)]
struct WorldFrame {
	pub pose: Pose<FixVec3, DQuat>,
	pub twist: Twist<FixVec3, DVec3>,	
}

#[derive(Component, Debug, Clone, Copy)]
struct LocalFrame {
	pub pose: Pose<FixVec3, DQuat>,
	pub twist: Twist<FixVec3, DVec3>,
}

//	Component Implementations
impl WorldFrame {
	pub fn to_local(&self, viewer: &WorldFrame) -> LocalFrame {
		//	Relative pose
		let rel_pos = FixVec3::new(
			self.pose.pos.x - viewer.pose.pos.x,
			self.pose.pos.y - viewer.pose.pos.y,
			self.pose.pos.z - viewer.pose.pos.z,
		);
		let rel_rot = viewer.pose.rot.inverse();
		
		let pos_local = rel_rot * rel_pos;
		let rot_local = rel_rot * self.pose.rot;

		let local_pose = Pose {
			pos: pos_local, 
			rot: rot_local,
		};
		
		//	Relative velocity
		let rel_v = FixVec3::new(
			self.twist.lin.x - viewer.twist.lin.x,
			self.twist.lin.y - viewer.twist.lin.y,
			self.twist.lin.z - viewer.twist.lin.z,
		);
		let rel_w = self.twist.ang - viewer.twist.ang;

		let local_twist = Twist {
			lin: rel_rot * rel_v,
			ang: rel_rot * rel_w,
		};

		//	Frame
		LocalFrame {
			pose: local_pose,
			twist: local_twist,
		}
	}
}

impl LocalFrame {
}

impl From<LocalFrame> for Transform {
	fn from(frame: LocalFrame) -> Self { Transform {
			translation: frame.pose.pos.as_vec3(),
			rotation: frame.pose.rot.as_quat(),
			scale: Vec3::ONE,
	}	}
}