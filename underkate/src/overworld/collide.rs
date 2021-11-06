use super::hitbox::Hitbox;
use super::move_trait::Position;
use super::passability_checker::PassabilityCheck;

pub trait Collide {
    fn can_move_to(&self, position: Position, pass: &impl PassabilityCheck) -> bool;

    fn find_passable_position(
        &self,
        orig_position: Position,
        assumed_new_position: Position,
        pass: &impl PassabilityCheck,
    ) -> Option<Position> {
        if self.can_move_to(assumed_new_position, pass) {
            return Some(assumed_new_position);
        }

        // Try moving only horizontally/vertically.
        let assumed_new_horizontal_position =
            Position::new(assumed_new_position.x, orig_position.y);
        if self.can_move_to(assumed_new_horizontal_position, pass) {
            return Some(assumed_new_horizontal_position);
        }

        let assumed_new_vertical_position = Position::new(orig_position.x, assumed_new_position.y);
        if self.can_move_to(assumed_new_vertical_position, pass) {
            return Some(assumed_new_vertical_position);
        }

        None
    }
}

impl<T: Hitbox> Collide for T {
    fn can_move_to(&self, position: Position, pass: &impl PassabilityCheck) -> bool {
        pass.can_pass(&self.hitbox_at(position))
    }
}
