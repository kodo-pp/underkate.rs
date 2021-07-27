pub mod texture;

use crate::geometry::ScreenPoint;
use ggez::{Context, GameResult};

pub trait Draw {
    fn draw(&mut self, ctx: &mut Context, center_at: ScreenPoint<f32>) -> GameResult;
}
