use std::marker::PhantomData;
use std::cmp::Ordering;

use image::{GenericImage, RgbaImage};

use images::{Image, Position, size::{MultipleOf, Size}};

pub struct GrayscalePositionFinder<S, SS> {
    grayscales: Vec<(Position, f64)>,
    phantom: PhantomData<(S, SS)>,
}

impl<S, SS> GrayscalePositionFinder<S, SS>
where
    S: MultipleOf<S>,
    SS: Size,
{
    pub fn new<I>(origin_image: I) -> GrayscalePositionFinder<S, SS>
    where
        I: Image<Size = S>,
    {
        let list = origin_image
            .split_into_pieces()
            .map(|piece| (piece.position(), piece.mean_grayscale()))
            .collect();
        GrayscalePositionFinder {
            grayscales: list,
            phantom: PhantomData,
        }
    }

    fn find_position<I>(&mut self, piece: &I) -> Position
    where
        I: Image<Size = SS>,
    {
        let piece_gray = piece.mean_grayscale();
        self.grayscales
            .iter()
            .map(|&(ref pos, ref f)| (*pos, f64::abs(f - piece_gray)))
            .max_by(|(_, dif1), (_, dif2)| dif1.partial_cmp(dif2).unwrap_or(Ordering::Equal))
            .unwrap()
            .0
    }
}
