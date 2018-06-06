use std::marker::PhantomData;

use image::{DynamicImage, GenericImage};

use super::{PieceImage, Image};

pub struct OriginImage<S> {
    image: RgbaImage,
    phantom: Phantom<S>,
}

impl<S: Size> OriginImage<S> {
    pub fn split_into_pieces<'a, SS>(
        &'a self,
        size: SS,
    ) -> impl Iterator<Item = OriginImagePiece<'a, S, SS>>
    where
        S: MultipleOf<SS>,
        SS: Size,
    {
        let num_x = <S as Size>::WIDTH / <SS as Size>::WIDTH;
        let num_y = <S as Size>::HEIGHT / <SS as Size>::HEIGHT;
        (0..num_y)
            .flat_map(|y| (0..num_x).map(|x| Position { x: x, y: y }))
            .map(|pos| OriginImagePiece {
                pos: pos,
                origin: self,
                phantom: Phantom,
            })
    }
}

impl<S: Size> Image for OriginImage<S> {
    fn image(&self) -> &RgbaImage {
        &self.image
    }
}

pub struct OriginImagePiece<'a, S, SS> {
    pos: Position,
    origin: &'a OriginImage<S>,
    size_phantom: Phantom<SS>,
}

impl<'a, S, SS> Image for OriginImagePiece<'a, S, SS>
where
    S: Size + Multiple<SS>,
    SS: Size,
{
    fn image(&self) -> &DynamicImage {
        hoge
    }
}

/*
pub trait OriginImage {
    type PieceImage: PieceImage;

    const WIDTH: u32; // Must be multible of Self::PieceImage::WIDTH
    const HEIGHT: u32; // Must be multible of Self::PieceImage::HEIGHT

    const NUM_X: u32 = Self::WIDTH / Self::PieceImage::WIDTH;
    const NUM_Y: u32 = Self::HEIGHT / Self::PieceImage::HEIGHT;
    const NUM_PIECE: u32 = Self::NUM_X * Self::NUM_Y;

    fn image(&self) -> &DynamicImage;

    fn piece_iter<'a>(&'a self) -> OriginImagePieceIter<Self>
    where
        Self: Sized,
    {
        OriginImagePieceIter {
            next_index: 0,
            origin: self,
        }
    }

    fn grayscale_list(&self) -> &GrayscaleList<Self>
    where
        Self: Sized;
}

pub struct OriginImagePieceIter<'a, O: 'a> {
    next_index: u32,
    origin: &'a O,
}

impl<'a, O> Iterator for OriginImagePieceIter<'a, O>
where
    O: 'a + OriginImage,
{
    type Item = OriginImagePiece<'a, O>;

    fn next(&mut self) -> Option<Self::Item> {
        if O::NUM_PIECE < self.next_index {
            return None;
        }
        let x = self.next_index % O::NUM_X;
        let y = self.next_index / O::NUM_Y;
        let pos = Position { x: x, y: y };

        self.next_index += 1;

        Some(OriginImagePiece {
            pos: pos,
            origin: self.origin,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

pub struct GrayscaleList<I> {
    mean_grayscales: Vec<f64>,
    phantom: PhantomData<I>,
}

impl<I: OriginImage> GrayscaleList<I> {
    pub fn new(origin: &I) -> GrayscaleList<I> {
        let mut gray_image = origin.image().to_luma();

        let mut mean_grayscales = Vec::with_capacity(I::NUM_PIECE as usize);

        for x in 0..I::NUM_Y {
            for y in 0..I::NUM_X {
                let cropped = gray_image.sub_image(
                    x * I::PieceImage::WIDTH,  // x
                    y * I::PieceImage::HEIGHT, // y
                    I::PieceImage::WIDTH,      // width
                    I::PieceImage::HEIGHT,     // height
                );
                let sum_gray = cropped
                    .pixels()
                    .fold(0f64, |sum, (_, _, p)| sum + (p.data[0] as f64));
                let mean_gray = sum_gray / I::NUM_PIECE as f64;
                mean_grayscales.push(mean_gray);
            }
        }

        GrayscaleList {
            mean_grayscales: mean_grayscales,
            phantom: PhantomData,
        }
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (Position, f64)> + 'a {
        self.mean_grayscales.iter().enumerate().map(|(i, f)| {
            let x = i as u32 % I::NUM_X;
            let y = i as u32 / I::NUM_X;
            let pos = Position { x: x, y: y };
            (pos, *f)
        })
    }

    pub fn best_match(&self, piece_grayscale: f64) -> Position {
        let init_pos = Position { x: 0, y: 0 };
        self.iter()
            .fold((init_pos, 0f64), |(acc_p, acc_dif), (p, f)| {
                let dif = f64::abs(piece_grayscale - f);
                if dif < acc_dif {
                    (p, dif)
                } else {
                    (acc_p, acc_dif)
                }
            })
            .0
    }
}
*/
