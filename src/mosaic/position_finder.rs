use std::{cmp::Ordering, collections::HashSet, marker::PhantomData};

use images::{Image, Position, size::{MultipleOf, Size}};

pub struct GrayscalePositionFinder<S, SS> {
    grayscales: Vec<(Position, f64)>,
    empty_pos: HashSet<Position>,
    phantom: PhantomData<(S, SS)>,
}

impl<S, SS> GrayscalePositionFinder<S, SS>
where
    S: MultipleOf<SS>,
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
            empty_pos: HashSet::new(),
            phantom: PhantomData,
        }
    }

    pub fn find_position<I>(&mut self, piece: &I) -> Position
    where
        I: Image<Size = SS>,
    {
        const ADDITION_TO_EMPTY_POS: f64 = 30f64;

        let piece_gray = piece.mean_grayscale();
        debug!("Mean grayscale of new piece is {}", piece_gray);
        let pos = self.grayscales
            .iter()
            .map(|&(ref pos, ref f)| {
                let mut distance = f64::abs(f - piece_gray);
                if self.empty_pos.contains(pos) {
                    distance += ADDITION_TO_EMPTY_POS;
                }
                (*pos, distance)
            })
            .min_by(|(_, dif1), (_, dif2)| dif1.partial_cmp(dif2).unwrap_or(Ordering::Equal))
            .unwrap()
            .0;
        self.empty_pos.remove(&pos);
        pos
    }
}
