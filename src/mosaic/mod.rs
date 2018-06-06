use std::marker::PhantomData;

use image::{GenericImage, Rgba, RgbaImage};

use images::{Image, Position, size::{Size, SmallerThan}};

pub mod insta_api;
pub mod position_finder;

pub struct MosaicArt<S> {
    image: RgbaImage,
    phantom: PhantomData<S>,
}

impl<S: Size> MosaicArt<S> {
    /// Create a clear MosaicArt.
    pub fn new() -> MosaicArt<S> {
        const CLEAR_PIXEL: Rgba<u8> = Rgba { data: [0, 0, 0, 0] };
        let clear_img = RgbaImage::from_pixel(S::WIDTH, S::HEIGHT, CLEAR_PIXEL);
        MosaicArt {
            image: clear_img,
            phantom: PhantomData,
        }
    }

    pub fn with_base<I>(base_img: I) -> MosaicArt<S>
    where
        I: Image<Size = S, Image = RgbaImage, Pixel = Rgba<u8>>,
    {
        MosaicArt {
            image: base_img.image().clone(),
            phantom: PhantomData,
        }
    }

    pub fn overpaint_by<I: Image>(&mut self, image: I, pos: Position)
    where
        I: Image<Image = RgbaImage, Pixel = Rgba<u8>>,
        I::Size: SmallerThan<S>,
    {
        self.image.copy_from(image.image(), pos.x, pos.y);
    }
}
