use images::{MultipleOf, Size, SizedImage, SmallerThan};
use post::{GenericPost, HashtagList, Post};
use super::{DistanceCalcAlgo, MosaicPieceVec};

pub struct MosaicArt<S, SS> {
    pub image: SizedImage<S>,
    pub posts: Vec<GenericPost<SS>>,
    pub hashtags: HashtagList,
}

pub struct MosaicArtGenerator<S, SS> {
    // immutable
    _origin_image: SizedImage<S>,
    hashtags: HashtagList,
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
        let hashtags = HashtagList::new(hashtags);
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

    pub fn hashtags(&self) -> HashtagList {
        self.hashtags.clone()
    }

    pub fn current_art(&self) -> MosaicArt<S, SS> {
        MosaicArt {
            image: self.current_art.clone(),
            posts: self.pieces.iter().map(|piece| piece.post.clone()).collect(),
            hashtags: self.hashtags.clone(),
        }
    }

    pub fn apply_post(&mut self, post: GenericPost<SS>) -> MosaicArt<S, SS> {
        // calc distance between each original image's pieces
        let piece = self.calc_algo.calc_post(post);
        let (pos, _replaced) = self.pieces.replace_piece(piece.clone());
        self.current_art.overpaint_by(piece.post.image(), pos);

        self.current_art()
    }

    pub fn has_enough_pieces(&self) -> bool {
        !self.pieces
            .distance_vec
            .iter()
            .any(|d| d == Distance::max_valud())
    }
}
