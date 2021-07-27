use crate::geometry::GenericDimensions;
use ggez::graphics::Image;

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
