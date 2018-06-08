pub mod size;
pub mod fetcher;

pub use self::size::{MultipleOf, Size, SmallerThan};
pub use self::fetcher::ImageFetcher;

use std::marker::PhantomData;

use image::{GenericImage, GenericImageView, Pixel, SubImage};


pub trait Image {
    type Size: Size;
    type Image: GenericImageView<Pixel = Self::Pixel>;
    type Pixel: Pixel<Subpixel = u8>;

    fn image(&self) -> &Self::Image;

    fn image_mut(&mut self) -> &mut Self::Image;

    fn mean_grayscale(&self) -> f64 {
        let img = ::image::imageops::grayscale(self.image());
        let sum_gray: f64 = img.iter().fold(0f64, |sum, i| sum + (*i as f64));
        sum_gray / (img.len() as f64)
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

    fn overpaint_by<I>(&mut self, image: I, pos: Position)
    where
        Self::Image: GenericImage,
        I: Image<Pixel = Self::Pixel>,
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

pub struct ImagePiece<'a, I, SS>
where
    I: Image + 'a,
{
    image: SubImage<&'a <I::Image as GenericImageView>::InnerImageView>,
    position: Position,
    phantom: PhantomData<SS>,
}

impl<'a, I, SS> ImagePiece<'a, I, SS>
where
    I: Image + 'a,
{
    pub fn position(&self) -> Position {
        self.position
    }
}

impl<'a, I, SS> Image for ImagePiece<'a, I, SS>
where
    I: Image + 'a,
    I::Size: MultipleOf<SS>,
    SS: Size,
{
    type Size = SS;
    type Image = SubImage<&'a <I::Image as GenericImageView>::InnerImageView>;
    type Pixel = I::Pixel;

    fn image(&self) -> &Self::Image {
        &self.image
    }

    fn image_mut(&mut self) -> &mut Self::Image {
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
    <I as Image>::Size: MultipleOf<SS>,
    SS: Size,
{
    type Item = ImagePiece<'a, I, SS>;

    fn next(&mut self) -> Option<Self::Item> {
        let num_x = I::Size::WIDTH / SS::WIDTH; // May be optimized
        let num_y = I::Size::HEIGHT / SS::HEIGHT; // May be optimized
        if num_x * num_y < self.next_index {
            return None;
        }
        let x = SS::WIDTH * (self.next_index % num_x);
        let y = SS::HEIGHT * (self.next_index / num_y);
        let sub_image = self.source.image().view(x, y, SS::WIDTH, SS::HEIGHT);
        self.next_index += 1;
        Some(ImagePiece {
            image: sub_image,
            position: Position { x: x, y: y },
            phantom: PhantomData,
        })
    }
}
