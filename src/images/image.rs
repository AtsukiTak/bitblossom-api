use std::{marker::PhantomData, ops::Deref};
use image::{FilterType, GenericImage, Pixel, Rgba, RgbaImage, imageops::resize, png::PNGEncoder};

use images::{MultipleOf, Size, SmallerThan};
use error::Error;

#[derive(Debug, Clone)]
pub struct SizedImage<S> {
    image: RgbaImage,
    _size: PhantomData<S>,
}

#[derive(Debug, Clone)]
pub struct InvalidSizeError {
    pub expect_h: usize,
    pub expect_w: usize,
    pub found_h: usize,
    pub found_w: usize,
}

impl<S: Size> SizedImage<S> {
    pub fn new(image: RgbaImage) -> Result<SizedImage<S>, Error> {
        if image.width() == S::WIDTH && image.height() == S::HEIGHT {
            Ok(SizedImage {
                image: image,
                _size: PhantomData,
            })
        } else {
            bail!(::error::ErrorKind::InvalidImageSize(S::WIDTH, S::HEIGHT));
        }
    }

    pub fn clear_image() -> SizedImage<S> {
        const CLEAR_PIXEL: Rgba<u8> = Rgba { data: [0, 0, 0, 0] };
        let clear_img = RgbaImage::from_pixel(S::WIDTH, S::HEIGHT, CLEAR_PIXEL);
        SizedImage {
            image: clear_img,
            _size: PhantomData,
        }
    }

    pub fn from_another_size(image: RgbaImage) -> SizedImage<S> {
        let resized = resize(&image, S::WIDTH, S::HEIGHT, FilterType::Lanczos3);
        SizedImage {
            image: resized,
            _size: PhantomData,
        }
    }

    pub fn from_raw_bytes(bytes: &[u8]) -> Result<SizedImage<S>, Error> {
        let img = ::image::load_from_memory(bytes)?.to_rgba();
        SizedImage::new(img)
    }

    pub fn from_another_image_raw_bytes(bytes: &[u8]) -> Result<SizedImage<S>, Error> {
        let img = ::image::load_from_memory(bytes)?.to_rgba();
        Ok(SizedImage::from_another_size(img))
    }

    pub fn to_png_bytes(&self) -> Vec<u8> {
        let mut vec = Vec::new();
        PNGEncoder::new(&mut vec)
            .encode(
                self.image.deref(),
                S::WIDTH,
                S::HEIGHT,
                Rgba::<u8>::color_type(),
            )
            .expect("Failed to encode into PNG");
        vec
    }

    pub fn mean_grayscale(&self) -> f64 {
        let img = ::image::imageops::grayscale(&self.image);
        let sum_gray: f64 = img.iter().fold(0f64, |sum, i| sum + (*i as f64));
        sum_gray / (img.len() as f64)
    }

    pub fn mean_alpha(&self) -> f64 {
        unimplemented!();
    }

    /// Fast crop function
    pub fn crop<SS>(&self, pos: Position) -> SizedImage<SS>
    where
        SS: Size + SmallerThan<S>,
    {
        let x = pos.x as usize;
        let y = pos.y as usize;
        let width = SS::WIDTH as usize;
        let height = SS::HEIGHT as usize;

        let mut vec: Vec<u8> = Vec::with_capacity(width * height * 4);
        unsafe {
            vec.set_len(width * height * 4);
        }
        let width_pixels = 4 * width;
        for i in 0..height {
            let source_bytes = {
                let y = y + i;
                let start_idx = 4 * (y * S::WIDTH as usize + x);
                let end_idx = start_idx + width_pixels;
                &self.image.deref()[start_idx..end_idx]
            };
            let dist_bytes = {
                let start_idx = i * width_pixels;
                let end_idx = start_idx + width_pixels;
                &mut vec.as_mut_slice()[start_idx..end_idx]
            };
            dist_bytes.copy_from_slice(source_bytes);
        }
        let img = RgbaImage::from_vec(width as u32, height as u32, vec).unwrap();
        SizedImage::new(img).unwrap()
    }

    pub fn split_into_pieces<'a, SS>(&'a self) -> ImagePieceIter<'a, S, SS>
    where
        S: MultipleOf<SS>,
        SS: Size + SmallerThan<S>,
    {
        ImagePieceIter {
            next_index: 0,
            source: self,
            phantom: PhantomData,
        }
    }

    pub fn overpaint_by<SS>(&mut self, image: &SizedImage<SS>, pos: Position)
    where
        SS: Size + SmallerThan<S>,
    {
        self.image.copy_from(&image.image, pos.x, pos.y);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

pub struct ImagePiece<SS> {
    pub image: SizedImage<SS>,
    pub position: Position,
}

pub struct ImagePieceIter<'a, S: 'a, SS> {
    next_index: u32,
    source: &'a SizedImage<S>,
    phantom: PhantomData<SS>,
}

impl<'a, S, SS> Iterator for ImagePieceIter<'a, S, SS>
where
    S: MultipleOf<SS>,
    SS: Size + SmallerThan<S>,
{
    type Item = ImagePiece<SS>;

    fn next(&mut self) -> Option<Self::Item> {
        let num_x = S::WIDTH / SS::WIDTH; // May be optimized
        let num_y = S::HEIGHT / SS::HEIGHT; // May be optimized
        let num_pieces = num_x * num_y;
        if self.next_index == num_pieces {
            return None;
        }
        let x = SS::WIDTH * (self.next_index % num_x);
        let y = SS::HEIGHT * (self.next_index / num_y);
        let pos = Position { x: x, y: y};
        let cropped = self.source.crop(pos.clone());
        self.next_index += 1;
        Some(ImagePiece {
            image: cropped,
            position: pos,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use images::size::{Size1500x1500, Size30x30};

    struct TestImg<S> {
        image: RgbaImage,
        size: PhantomData<S>,
    }

    impl<S: Size> TestImg<S> {
        pub fn new() -> TestImg<S> {
            TestImg {
                image: RgbaImage::new(S::WIDTH, S::HEIGHT),
                size: PhantomData,
            }
        }
    }

    impl<S: Size> Image for TestImg<S> {
        type Size = S;
        fn image(&self) -> &RgbaImage {
            &self.image
        }
        fn image_mut(&mut self) -> &mut RgbaImage {
            &mut self.image
        }
    }

    #[test]
    fn crop_big_image() {
        let blank_img = TestImg::<Size1500x1500>::new();
        for img in blank_img.split_into_pieces::<Size30x30>() {
            assert_eq!(img.image().width(), 30);
            assert_eq!(img.image().height(), 30);
        }
    }
}
