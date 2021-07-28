use super::Draw;
use crate::geometry::{OnScreen, ScreenPoint};
use ggez::graphics::{self, DrawParam, Image};
use ggez::{Context, GameResult};

pub struct Texture {
    image: Image,
    scale_factor: f32,
}

impl Texture {
    pub fn new_static(image: Image, scale_factor: f32) -> Texture {
        Texture {
            image,
            scale_factor,
        }
    }
}

impl AsRef<Image> for Texture {
    fn as_ref(&self) -> &Image {
        &self.image
    }
}

impl Draw for Texture {
    fn draw(&self, ctx: &mut Context, center_at: ScreenPoint<f32>) -> GameResult {
        let scale_vector = [self.scale_factor; 2];
        graphics::draw(
            ctx,
            self.as_ref(),
            DrawParam::new()
                .dest(center_at.on_screen())
                .scale(scale_vector)
                .offset([0.5, 0.5]),
        )
    }
}
