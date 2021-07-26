use ggez::{Context, GameResult};

pub trait Draw {
    fn draw(&mut self, ctx: &mut Context) -> GameResult;
}
