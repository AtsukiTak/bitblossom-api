use std::sync::Arc;

use images::{MultipleOf, Size, SizedImage, Image, SmallerThan};
use insta::{InstaPostInfo, InstaPost};
use super::{DistanceCalcAlgo, MosaicPieceVec};

pub struct MosaicArt {
    pub image: Image,
    pub posts: Vec<InstaPostInfo>,
    pub hashtags: Arc<Vec<String>>,
}

pub struct MosaicArtGenerator<S, SS> {
    // immutable
    _origin_image: SizedImage<S>,
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
            _origin_image: origin,
            hashtags: hashtags,
            calc_algo: calc_algo,
            current_art: current_art,
            pieces: pieces,
        }
    }

    pub fn hashtags(&self) -> Arc<Vec<String>> {
        self.hashtags.clone()
    }

    pub fn current_art(&self) -> MosaicArt {
        MosaicArt {
            image: self.current_art.image.clone(),
            posts: self.pieces
                .iter()
                .map(|piece| piece.insta_post.meta.clone())
                .collect(),
            hashtags: self.hashtags.clone(),
        }
    }

    pub fn apply_post(&mut self, post: InstaPost<SS>) -> MosaicArt {
        // calc distance between each original image's pieces
        let piece = self.calc_algo.calc_post(post);
        let (pos, _replaced) = self.pieces.replace_piece(piece.clone());
        self.current_art.overpaint_by(piece.get_image(), pos);

        self.current_art()
    }
}
