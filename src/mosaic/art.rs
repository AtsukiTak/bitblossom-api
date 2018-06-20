use std::sync::{Arc, Mutex};

use images::{SizedImage, size::{MultipleOf, Size, SmallerThan}};
use insta::InstaPost;
use super::{MosaicPiece, MosaicPieceVec};

#[derive(Debug)]
pub struct MosaicArt<S, SS> {
    image: SizedImage<S>,
    pieces: MosaicPieceVec<S, SS>,
    hashtags: Vec<String>,
}

impl<S, SS> MosaicArt<S, SS>
where
    S: Size + MultipleOf<SS>,
    SS: Size + SmallerThan<S>,
{
    pub fn new(hashtags: Vec<String>, origin: SizedImage<S>) -> MosaicArt<S, SS> {
        MosaicArt {
            image: SizedImage::clear_image(),
            pieces: MosaicPieceVec::with_origin_image(origin),
            hashtags: hashtags,
        }
    }

    pub fn apply_piece(&mut self, piece: MosaicPiece<SS>) {
        let (pos, _replaced_piece) = self.pieces.replace_piece(piece.clone());
        self.image.overpaint_by(piece.get_image(), pos);
    }

    pub fn get_image(&self) -> &SizedImage<S> {
        &self.image
    }

    pub fn get_hashtags(&self) -> &[String] {
        self.hashtags.as_slice()
    }
}

#[derive(Clone)]
pub struct SharedMosaicArt<S, SS>(Arc<Mutex<MosaicArt<S, SS>>>);

impl<S, SS> SharedMosaicArt<S, SS>
where
    S: Size + MultipleOf<SS>,
    SS: Size + SmallerThan<S>,
{
    pub fn new(hashtags: Vec<String>, origin: SizedImage<S>) -> SharedMosaicArt<S, SS> {
        SharedMosaicArt(Arc::new(Mutex::new(MosaicArt::new(hashtags, origin))))
    }

    pub fn apply_piece(&self, piece: MosaicPiece<SS>) {
        self.0.lock().unwrap().apply_piece(piece);
    }

    pub fn get_image(&self) -> SizedImage<S> {
        self.0.lock().unwrap().image.clone()
    }

    pub fn get_hashtags(&self) -> Vec<String> {
        self.0.lock().unwrap().hashtags.clone()
    }

    pub fn get_piece_posts(&self) -> Vec<InstaPost<SS>> {
        let art = self.0.lock().unwrap();
        let mut vec = Vec::new();
        for piece in art.pieces.iter() {
            vec.push(piece.insta_post.clone());
        }
        vec
    }
}
