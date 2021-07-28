use super::move_trait::Move;
use crate::geometry::OverworldVector;
use std::time::Duration;

#[derive(Debug, Copy, Clone)]
pub struct WalkInfo {
    pub velocity: OverworldVector<f32>,
}

#[derive(Debug, Copy, Clone)]
pub enum WalkState {
    Still,
    Walking(WalkInfo),
}

impl Default for WalkState {
    fn default() -> Self {
        WalkState::Still
    }
}

pub trait Walk: Move {
    fn walk_state(&self) -> WalkState;
    fn set_walk_state(&mut self, walk_state: WalkState);

    fn start_walking(&mut self, walk_info: WalkInfo) {
        self.set_walk_state(WalkState::Walking(walk_info));
    }

    fn stop_walking(&mut self) {
        self.set_walk_state(WalkState::Still);
    }

    fn update_position(&mut self, time_slice: Duration) {
        match self.walk_state() {
            WalkState::Still => (),
            WalkState::Walking(WalkInfo { velocity, .. }) => {
                let position_change = velocity * time_slice.as_secs_f32();
                self.set_position(self.position() + position_change);
            }
        }
    }
}
