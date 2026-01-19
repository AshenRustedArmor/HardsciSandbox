//  	Imports
//	Bevy base
use bevy::prelude::*;
use bevy::math::{DVec3, DQuat};

/* Component Implementations
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
} // */