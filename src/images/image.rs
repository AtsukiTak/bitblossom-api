use std::{intrinsics::copy_nonoverlapping, marker::PhantomData, ops::Deref};
use image::{GenericImage, Pixel, Rgba, RgbaImage, png::PNGEncoder};

use images::{Size, MultipleOf, SmallerThan};

pub trait Image {
    type Size: Size;

    fn image(&self) -> &RgbaImage;

    fn image_mut(&mut self) -> &mut RgbaImage;

    fn to_png_bytes(&self) -> Vec<u8> {
        let mut vec = Vec::new();
        PNGEncoder::new(&mut vec).encode(
            self.image().deref(),
            Self::Size::WIDTH,
            Self::Size::HEIGHT,
            Rgba::<u8>::color_type(),
        ).expect("Failed to encode into PNG");
        vec
    }

    fn mean_grayscale(&self) -> f64 {
        let img = ::image::imageops::grayscale(self.image());
        let sum_gray: f64 = img.iter().fold(0f64, |sum, i| sum + (*i as f64));
        sum_gray / (img.len() as f64)
    }

    /// Fast crop function
    fn crop(&self, x: u32, y: u32, width: u32, height: u32) -> RgbaImage {
        let x = x as usize;
        let y = y as usize;
        let width = width as usize;
        let height = height as usize;

        let mut vec: Vec<u8> = Vec::with_capacity(width * height * 4);
        unsafe {
            vec.set_len(width * height * 4);
        }
        let width_pixels = 4 * width;
        for i in 0..height {
            let source_bytes = {
                let y = y + i;
                let start_idx = 4 * (y * Self::Size::WIDTH as usize + x);
                let end_idx = start_idx + width_pixels;
                self.image().deref()[start_idx..end_idx].as_ptr()
            };
            let dist_bytes = {
                let start_idx = i * width_pixels;
                let end_idx = start_idx + width_pixels;
                vec.as_mut_slice()[start_idx..end_idx].as_mut_ptr()
            };
            unsafe {
                copy_nonoverlapping(source_bytes, dist_bytes, width_pixels);
            }
        }
        RgbaImage::from_vec(width as u32, height as u32, vec).unwrap()
    }

    fn split_into_pieces<'a, SS>(&'a self) -> ImagePieceIter<'a, Self, SS>
    where
        Self: Sized,
        Self::Size: MultipleOf<SS>,
        SS: Size,
    {
        ImagePieceIter {
            next_index: 0,
            source: self,
            phantom: PhantomData,
        }
    }

    fn overpaint_by<I>(&mut self, image: &I, pos: Position)
    where
        I: Image,
        I::Size: SmallerThan<Self::Size>,
    {
        self.image_mut().copy_from(image.image(), pos.x, pos.y);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

pub struct ImagePiece<SS> {
    image: RgbaImage,
    position: Position,
    phantom: PhantomData<SS>,
}

impl<SS> ImagePiece<SS> {
    pub fn position(&self) -> Position {
        self.position
    }
}

impl<SS> Image for ImagePiece<SS>
where
    SS: Size,
{
    type Size = SS;

    fn image(&self) -> &RgbaImage {
        &self.image
    }

    fn image_mut(&mut self) -> &mut RgbaImage {
        &mut self.image
    }
}

pub struct ImagePieceIter<'a, I: 'a, SS> {
    next_index: u32,
    source: &'a I,
    phantom: PhantomData<SS>,
}

impl<'a, I, SS> Iterator for ImagePieceIter<'a, I, SS>
where
    I: Image,
    I::Size: MultipleOf<SS>,
    SS: Size,
{
    type Item = ImagePiece<SS>;

    fn next(&mut self) -> Option<Self::Item> {
        let num_x = I::Size::WIDTH / SS::WIDTH; // May be optimized
        let num_y = I::Size::HEIGHT / SS::HEIGHT; // May be optimized
        let num_pieces = num_x * num_y;
        if self.next_index == num_pieces {
            return None;
        }
        let x = SS::WIDTH * (self.next_index % num_x);
        let y = SS::HEIGHT * (self.next_index / num_y);
        let cropped = self.source.crop(x, y, SS::WIDTH, SS::HEIGHT);
        self.next_index += 1;
        Some(ImagePiece {
            image: cropped,
            position: Position { x: x, y: y },
            phantom: PhantomData,
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
