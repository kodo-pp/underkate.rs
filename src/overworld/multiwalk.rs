use super::walk::{Walk, WalkInfo};
use crate::geometry::OverworldVector;
use paste::paste;

#[derive(Debug, Copy, Clone, Default)]
pub struct MultiWalkState {
    forward_velocity: Option<f32>,
    backward_velocity: Option<f32>,
    left_velocity: Option<f32>,
    right_velocity: Option<f32>,
}

impl MultiWalkState {
    fn resulting_velocity(&self) -> OverworldVector<f32> {
        // TODO: use std::array::IntoIter when it is stabilized.
        [
            (self.forward_velocity, 0.0, -1.0),
            (self.backward_velocity, 0.0, 1.0),
            (self.left_velocity, -1.0, 0.0),
            (self.right_velocity, 1.0, 0.0),
        ]
        .iter()
        .copied()
        .map(|(velocity_abs, x_coeff, y_coeff)| {
            OverworldVector::new(x_coeff, y_coeff) * velocity_abs.unwrap_or(0.0)
        })
        .sum()
    }

    fn is_still(&self) -> bool {
        [
            self.forward_velocity,
            self.backward_velocity,
            self.left_velocity,
            self.right_velocity,
        ]
        .iter()
        .all(Option::is_none)
    }
}

macro_rules! gen_methods_for_direction {
    ($direction:ident) => {
        paste! {
            fn [< start_walking_ $direction >](&mut self, velocity: f32) {
                let mut state = self.multi_walk_state();
                state.[< $direction _velocity >] = Some(velocity);
                self.set_multi_walk_state(state);
                self.update_walk_state();
            }

            fn [< stop_walking_ $direction >](&mut self) {
                let mut state = self.multi_walk_state();
                state.[< $direction _velocity >] = None;
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
