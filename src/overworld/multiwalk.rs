use super::walk::{Walk, WalkInfo};
use crate::geometry::OverworldVector;
use paste::paste;

#[derive(Debug, Copy, Clone)]
pub struct MultiWalkState {
    velocity_abs: f32,
    walking_forward: bool,
    walking_backward: bool,
    walking_left: bool,
    walking_right: bool,
}

impl MultiWalkState {
    pub fn new(velocity_abs: f32) -> Self {
        Self { velocity_abs, walking_forward: false, walking_backward: false, walking_left: false, walking_right: false }
    }
    pub fn resulting_velocity(&self) -> OverworldVector<f32> {
        // TODO: use std::array::IntoIter when it is stabilized.
        [
            (self.walking_forward, 0.0, -1.0),
            (self.walking_backward, 0.0, 1.0),
            (self.walking_left, -1.0, 0.0),
            (self.walking_right, 1.0, 0.0),
        ]
        .iter()
        .copied()
        .map(|(enabled, x_coeff, y_coeff)| {
            OverworldVector::new(x_coeff, y_coeff) * self.velocity_abs * if enabled { 1.0 } else { 0.0 }
        })
        .sum()
    }

    pub fn is_still(&self) -> bool {
        let horizontal_still = self.walking_forward == self.walking_backward;
        let vertical_still = self.walking_left == self.walking_right;
        horizontal_still && vertical_still
    }
}

macro_rules! gen_methods_for_direction {
    ($direction:ident) => {
        paste! {
            fn [< start_walking_ $direction >](&mut self) {
                let mut state = self.multi_walk_state();
                state.[< walking_ $direction >] = true;
                self.set_multi_walk_state(state);
                self.update_walk_state();
            }

            fn [< stop_walking_ $direction >](&mut self) {
                let mut state = self.multi_walk_state();
                state.[< walking_ $direction >] = false;
                self.set_multi_walk_state(state);
                self.update_walk_state();
            }
        }
    };
}

pub trait MultiWalk: Walk {
    fn multi_walk_state(&self) -> MultiWalkState;
    fn set_multi_walk_state(&mut self, multi_walk_state: MultiWalkState);

    fn update_walk_state(&mut self) {
        let state = self.multi_walk_state();

        if state.is_still() {
            self.stop_walking();
        } else {
            self.start_walking(WalkInfo {
                velocity: state.resulting_velocity(),
            })
        }
    }

    gen_methods_for_direction!(forward);
    gen_methods_for_direction!(backward);
    gen_methods_for_direction!(left);
    gen_methods_for_direction!(right);
}
