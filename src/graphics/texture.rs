use super::Draw;
use crate::geometry::{OnScreen, ScreenPoint};
use ggez::graphics::{self, DrawParam, Image};
use ggez::{Context, GameResult};
use std::time::{Duration, Instant};

enum TextureKind {
    Static(Image),
    Animated {
        frames: Vec<Image>,
        frame_interval: Duration,
        start_instant: Instant,
    },
}

pub struct Texture {
    kind: TextureKind,
    scale_factor: f32,
}

impl Texture {
    pub fn new_static(image: Image, scale_factor: f32) -> Texture {
        Texture {
            kind: TextureKind::Static(image),
            scale_factor,
        }
    }

    pub fn new_animated(
        frames: Vec<Image>,
        frame_interval: Duration,
        scale_factor: f32,
    ) -> Texture {
        Texture {
            kind: TextureKind::Animated {
                frames,
                frame_interval,
                start_instant: Instant::now(),
            },
            scale_factor,
        }
    }

    fn image_for_now(&self) -> &Image {
        match &self.kind {
            TextureKind::Static(ref image) => &image,
            TextureKind::Animated {
                frames,
                frame_interval,
                start_instant,
            } => {
                let time_passed_since_start = start_instant.elapsed();
                let frames_passed = (time_passed_since_start.as_secs_f64()
                    / frame_interval.as_secs_f64())
                .floor() as usize;
                let frame_index = frames_passed % frames.len();
                &frames[frame_index]
            }
        }
    }
}

impl Draw for Texture {
    fn draw(&self, ctx: &mut Context, center_at: ScreenPoint<f32>) -> GameResult {
        let scale_vector = [self.scale_factor; 2];
        graphics::draw(
            ctx,
            self.image_for_now(),
            DrawParam::new()
                .dest(center_at.on_screen())
                .scale(scale_vector)
                .offset([0.5, 0.5]),
        )
    }
}
