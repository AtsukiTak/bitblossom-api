use std::{marker::PhantomData, ops::{Deref, DerefMut}};
use image::{FilterType, GenericImage, Pixel, Rgba, RgbaImage, imageops::resize, png::PNGEncoder};

use images::{MultipleOf, Size, SmallerThan};
use error::Error;

#[derive(Debug, Clone)]
pub struct Image {
    raw: RgbaImage,
}

impl Image {
    pub fn new(raw: RgbaImage) -> Image {
        Image { raw: raw }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Image, Error> {
        Ok(Image::new(::image::load_from_memory(bytes)?.to_rgba()))
    }

    pub fn clear_image(width: u32, height: u32) -> Image {
        const CLEAR_PIXEL: Rgba<u8> = Rgba { data: [0, 0, 0, 0] };
        let clear_img = RgbaImage::from_pixel(width, height, CLEAR_PIXEL);
        Image::new(clear_img)
    }

    pub fn to_png_bytes(&self) -> Vec<u8> {
        let mut vec = Vec::new();
        PNGEncoder::new(&mut vec)
            .encode(
                self.raw.deref(),
                self.raw.width(),
                self.raw.height(),
                Rgba::<u8>::color_type(),
            )
            .expect("Failed to encode into PNG");
        vec
    }

    pub fn resize(&self, width: u32, height: u32) -> Image {
        Image::new(resize(&self.raw, width, height, FilterType::Lanczos3))
    }

    pub fn mean_grayscale(&self) -> f64 {
        let img = ::image::imageops::grayscale(&self.raw);
        let sum_gray: f64 = img.iter().fold(0f64, |sum, i| sum + (*i as f64));
        sum_gray / (img.len() as f64)
    }

    pub fn mean_alpha(&self) -> f64 {
        let alpha_iter = self.raw.chunks(4).map(|chunk| chunk[3]);
        let sum_alpha = alpha_iter.fold(0u64, |sum, a| sum + (a as u64));
        sum_alpha as f64 / (self.raw.len() / 4) as f64
    }
}

impl Deref for Image {
    type Target = RgbaImage;
    fn deref(&self) -> &RgbaImage {
        &self.raw
    }
}

impl DerefMut for Image {
    fn deref_mut(&mut self) -> &mut RgbaImage {
        &mut self.raw
    }
}

#[derive(Debug, Clone)]
pub struct SizedImage<S> {
    pub image: Image,
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
    pub fn new(image: Image) -> Result<SizedImage<S>, Error> {
        if image.width() == S::WIDTH && image.height() == S::HEIGHT {
            Ok(SizedImage {
                image: image,
                _size: PhantomData,
            })
        } else {
            bail!(::error::ErrorKind::InvalidImageSize(S::WIDTH, S::HEIGHT));
        }
    }

    pub fn with_resize(image: Image) -> SizedImage<S> {
        SizedImage::new(image.resize(S::WIDTH, S::HEIGHT)).unwrap()
    }

    pub fn clear_image() -> SizedImage<S> {
        SizedImage {
            image: Image::clear_image(S::WIDTH, S::HEIGHT),
            _size: PhantomData,
        }
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
                &self.image.raw.deref()[start_idx..end_idx]
            };
            let dist_bytes = {
                let start_idx = i * width_pixels;
                let end_idx = start_idx + width_pixels;
                &mut vec.as_mut_slice()[start_idx..end_idx]
            };
            dist_bytes.copy_from_slice(source_bytes);
        }
        let img = Image::new(RgbaImage::from_vec(width as u32, height as u32, vec).unwrap());
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
        self.image.raw.copy_from(&image.image.raw, pos.x, pos.y);
    }
}

impl<S: Size> Deref for SizedImage<S> {
    type Target = Image;
    fn deref(&self) -> &Image {
        &self.image
    }
}

impl<S: Size> DerefMut for SizedImage<S> {
    fn deref_mut(&mut self) -> &mut Image {
        &mut self.image
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
        let pos = Position { x: x, y: y };
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

    fn blank_1500x1500_img() -> SizedImage<Size1500x1500> {
        SizedImage::new(RgbaImage::new(1500, 1500)).unwrap()
    }

    fn white_30x30_img() -> SizedImage<Size30x30> {
        let white_pixel = Rgba {
            data: [255, 255, 255, 255],
        };
        let img = RgbaImage::from_pixel(30, 30, white_pixel);
        SizedImage::new(img).unwrap()
    }

    #[test]
    fn crop_big_image() {
        let blank_img = blank_1500x1500_img();
        for piece in blank_img.split_into_pieces::<Size30x30>() {
            assert_eq!(piece.image.image.width(), 30);
            assert_eq!(piece.image.image.height(), 30);
        }
    }

    #[test]
    fn mean_alpha() {
        let mut blank_img = blank_1500x1500_img();
        assert_eq!(blank_img.mean_alpha(), 0f64);

        let white_img = white_30x30_img();
        blank_img.overpaint_by(&white_img, Position { x: 0, y: 0 });
        assert_eq!(blank_img.mean_alpha(), 255f64 / (50f64 * 50f64));
    }
}
