use super::hitbox::Hitbox;
use super::passability_checker::PassabilityChecker;
use crate::geometry::OverworldPoint;

pub trait Collide {
    fn can_move_to(&self, position: OverworldPoint<f32>, pass: &PassabilityChecker) -> bool;
}

impl<T: Hitbox> Collide for T {
    fn can_move_to(&self, position: OverworldPoint<f32>, pass: &PassabilityChecker) -> bool {
        pass.can_pass(&self.hitbox_at(position))
    }
}
