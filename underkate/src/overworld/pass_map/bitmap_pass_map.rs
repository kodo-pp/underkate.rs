use super::PassMap;
use crate::geometry::OverworldRect;

#[derive(Clone, Copy)]
pub struct Bitmap {
    width: usize,
    height: usize,
    data: &'static [u8],
}

impl Bitmap {
    pub fn new(width: usize, height: usize, data: &'static [u8]) -> Self {
        let expected_size = width.checked_mul(height).expect("Size overflow");
        if data.len() != expected_size {
            panic!(
                "Size mismatch: expected size {}, but the real size is {}",
                expected_size,
                data.len()
            );
        }
        Self {
            width,
            height,
            data,
        }
    }

    fn index(&self, x: usize, y: usize) -> usize {
        y.checked_mul(self.width)
            .and_then(|index| index.checked_add(x))
            .expect("Size overflow")
    }

    unsafe fn index_unchecked(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub unsafe fn pixel_occupied_unchecked(&self, x: usize, y: usize) -> bool {
        let index = self.index_unchecked(x, y);
        *self.data.get_unchecked(index) == 0
    }

    pub unsafe fn rect_occupied_unchecked(&self, rect: &OverworldRect<usize>) -> bool {
        for y in rect.y_range() {
            for x in rect.x_range() {
                if self.pixel_occupied_unchecked(x, y) {
                    return true;
                }
            }
        }
        false
    }

    pub fn rect_occupied(&self, rect: &OverworldRect<usize>) -> bool {
        assert!(rect.min.x <= rect.max.x);
        assert!(rect.min.y <= rect.max.y);
        let min_index = self.index(rect.min.x, rect.min.y);
        let max_index = self.index(rect.max.x, rect.max.y);
        assert!(min_index <= max_index);
        assert!(max_index < self.data.len());
        unsafe { self.rect_occupied_unchecked(rect) }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

#[derive(Clone, Copy)]
pub struct BitmapPassMap {
    bitmap: Bitmap,
    scale_recip: f32,
}

impl BitmapPassMap {
    pub fn new(bitmap: Bitmap, scale: f32) -> Self {
        Self {
            bitmap,
            scale_recip: scale.recip(),
        }
    }
}

impl PassMap for BitmapPassMap {
    fn collides_with(&self, rect: &OverworldRect<f32>) -> bool {
        let scaled = rect.scale(self.scale_recip, self.scale_recip);
        self.bitmap.rect_occupied(&scaled.round().to_usize())
    }

    fn bounds(&self) -> OverworldRect<f32> {
        OverworldRect::from_size([self.bitmap.width(), self.bitmap.height()].into()).to_f32()
    }
}
