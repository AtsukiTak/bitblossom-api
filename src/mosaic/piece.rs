use std::{marker::PhantomData, mem::replace};
use images::{MultipleOf, Position, Size, SizedImage, SmallerThan};
use post::GenericPost;
use super::Distance;

#[derive(Clone, Debug)]
pub struct MosaicPiece<SS> {
    pub post: GenericPost<SS>,
    // Distance between each origin pieces.
    pub(super) distance_vec: Vec<Distance>,
}

#[derive(Clone, Debug)]
pub struct MosaicPieceVec<S, SS> {
    pieces: Vec<(Distance, Option<MosaicPiece<SS>>)>,
    _origin_size: PhantomData<S>,
}

impl<S, SS> MosaicPieceVec<S, SS>
where
    S: Size + MultipleOf<SS>,
    SS: Size + SmallerThan<S>,
{
    pub fn iter(&self) -> impl Iterator<Item = &MosaicPiece<SS>> {
        self.pieces.iter().filter_map(|(_, opt)| opt.as_ref())
    }

    // Returns MosaicPieceVec based on origin image.
    // Piece whose alpha is 1.0 in origin image never be Some.
    pub fn with_origin_image(origin: &SizedImage<S>) -> MosaicPieceVec<S, SS> {
        let piece_n = (S::WIDTH * S::HEIGHT) / (SS::WIDTH * SS::HEIGHT);
        let mut pieces = vec![(Distance::max_value(), None); piece_n as usize];

        for (idx, p) in origin.split_into_pieces().enumerate() {
            if p.image.mean_alpha() == 0.0 {
                pieces[idx].0 = Distance::min_value();
            }
        }

        MosaicPieceVec {
            pieces: pieces,
            _origin_size: PhantomData,
        }
    }

    // Replace a piece with given piece.
    // Replaced piece is chosen as such replacing make mosaic art better.
    pub fn replace_piece(&mut self, piece: MosaicPiece<SS>) -> (Position, Option<MosaicPiece<SS>>) {
        let idx = {
            // Distance between origin pieces and current mosaic art's each piece
            let distances_curr = self.pieces.iter().map(|(d, _)| d);
            // Distance between origin pieces and new piece
            let distances_new = piece.distance_vec.iter();

            distances_curr
                .zip(distances_new)
                .map(|(curr_dist, new_dist)| {
                    if curr_dist < new_dist {
                        0
                    } else {
                        curr_dist - new_dist
                    }
                })
                .enumerate()
                .max_by_key(|(_i, gap)| *gap)
                .unwrap()
                .0
        };
        debug!("Replace index : {}", idx);

        // Replace old piece with new piece.
        let (_, old_piece) = replace(
            unsafe { self.pieces.get_unchecked_mut(idx) },
            (piece.distance_vec[idx], Some(piece)),
        );

        (Self::index_to_pos(idx), old_piece)
    }

    fn index_to_pos(idx: usize) -> Position {
        let nx = S::WIDTH / SS::WIDTH;
        let ny = S::HEIGHT / SS::WIDTH;
        let x = (idx as u32 % nx) * SS::WIDTH;
        let y = (idx as u32 / ny) * SS::HEIGHT;
        Position { x: x, y: y }
    }
}
