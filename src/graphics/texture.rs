use super::Draw;
use crate::geometry::{GenericDimensions, OnScreen, ScreenPoint};
use ggez::graphics::{self, DrawParam, Image};
use ggez::{Context, GameResult};

pub struct Texture {
    image: Image,
}

impl Texture {
    fn dimensions(&self) -> GenericDimensions<u32> {
        let width = self.image.width() as u32;
        let height = self.image.width() as u32;
        [width, height].into()
    }
}

impl From<Image> for Texture {
    fn from(image: Image) -> Texture {
        Texture { image }
    }
}

impl Into<Image> for Texture {
    fn into(self) -> Image {
        self.image
    }
}

impl AsRef<Image> for Texture {
    fn as_ref(&self) -> &Image {
        &self.image
    }
}

impl Draw for Texture {
    fn draw(&mut self, ctx: &mut Context, center_at: ScreenPoint<f32>) -> GameResult {
        graphics::draw(
            ctx,
            self.as_ref(),
            DrawParam::new().dest(center_at.on_screen()),
        )
    }
}
