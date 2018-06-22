use std::sync::Arc;

use images::{MultipleOf, Size, SizedImage, SmallerThan};
use insta::InstaPost;
use super::{DistanceCalcAlgo, MosaicPieceVec};

pub struct MosaicArt<S, SS> {
    pub image: SizedImage<S>,
    pub pieces: Vec<InstaPost<SS>>,
    pub hashtags: Arc<Vec<String>>,
}

pub struct MosaicArtGenerator<S, SS> {
    // immutable
    origin_image: SizedImage<S>,
    hashtags: Arc<Vec<String>>,
    calc_algo: DistanceCalcAlgo<S, SS>,

    // mutable
    current_art: SizedImage<S>,
    pieces: MosaicPieceVec<S, SS>,
}

impl<S, SS> MosaicArtGenerator<S, SS>
where
    S: Size + MultipleOf<SS>,
    SS: Size + SmallerThan<S>,
{
    pub fn new(origin: SizedImage<S>, hashtags: Vec<String>) -> MosaicArtGenerator<S, SS> {
        let hashtags = Arc::new(hashtags);
        let current_art = SizedImage::clear_image();
        let pieces = MosaicPieceVec::with_origin_image(&origin);
        let calc_algo = DistanceCalcAlgo::new(&origin);
        MosaicArtGenerator {
            origin_image: origin,
            hashtags: hashtags,
            calc_algo: calc_algo,
            current_art: current_art,
            pieces: pieces,
        }
    }

    pub fn hashtags(&self) -> Arc<Vec<String>> {
        self.hashtags.clone()
    }

    pub fn current_art(&self) -> MosaicArt<S, SS> {
        MosaicArt {
            image: self.current_art.clone(),
            pieces: self.pieces
                .iter()
                .map(|piece| piece.insta_post.clone())
                .collect(),
            hashtags: self.hashtags.clone(),
        }
    }

    pub fn apply_post(&mut self, post: InstaPost<SS>) -> MosaicArt<S, SS> {
        // calc distance between each original image's pieces
        let piece = self.calc_algo.calc_post(post);
        let (pos, _replaced) = self.pieces.replace_piece(piece.clone());
        self.current_art.overpaint_by(piece.get_image(), pos);

        self.current_art()
    }
}
