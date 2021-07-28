use crate::geometry::OverworldRect;

pub struct PassabilityChecker {
    map_rect: OverworldRect<f32>,
}

impl PassabilityChecker {
    pub fn new(map_rect: OverworldRect<f32>) -> Self {
        Self { map_rect }
    }

    pub fn can_pass(&self, hitbox: &OverworldRect<f32>) -> bool {
        self.map_rect.contains_box(hitbox)
    }
}
