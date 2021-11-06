use crate::geometry::OverworldRect;
use crate::overworld::pass_map::PassMap;

pub trait PassabilityCheck {
    fn can_pass(&self, player_hitbox: &OverworldRect<f32>) -> bool;
}

#[derive(Debug, Clone, Copy)]
pub struct PassMapPassabilityChecker<'passmap, P: PassMap> {
    pass_map: &'passmap P,
}

impl<'passmap, P: PassMap> PassMapPassabilityChecker<'passmap, P> {
    pub fn new(pass_map: &'passmap P) -> Self {
        Self { pass_map }
    }
}

impl<P: PassMap> PassabilityCheck for PassMapPassabilityChecker<'_, P> {
    fn can_pass(&self, player_hitbox: &OverworldRect<f32>) -> bool {
        let within_rect = || self.pass_map.bounds().contains_box(player_hitbox);
        let map_passable = || !self.pass_map.collides_with(player_hitbox);
        within_rect() && map_passable()
    }
}
