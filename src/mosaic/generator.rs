use images::{MultipleOf, Size, SizedImage, SmallerThan};
use post::{GenericPost, HashtagList, Post};
use util::{Id, IdGenerator};
use super::{Distance, DistanceFunc, MeanGrayscale, MosaicPiece, MosaicPieceVec};

pub struct MosaicArt<S, SS> {
    pub id: Id, // Used by api_server to determine whether rerurn cached response or construct a new response.
    pub image: SizedImage<S>,
    pub posts: Vec<GenericPost<SS>>,
    pub hashtags: HashtagList,
}

impl<S, SS> MosaicArt<S, SS> {
    fn new(
        id: Id,
        image: SizedImage<S>,
        posts: Vec<GenericPost<SS>>,
        hashtags: HashtagList,
    ) -> MosaicArt<S, SS> {
        MosaicArt {
            id: id,
            image: image,
            posts: posts,
            hashtags: hashtags,
        }
    }
}

pub struct MosaicArtGenerator<S, SS, D = MeanGrayscale<S, SS>> {
    // immutable
    _origin_image: SizedImage<S>,
    hashtags: HashtagList,
    distance_f: D,
    id_gen: IdGenerator,

    // mutable
    current_img: SizedImage<S>,
    pieces: MosaicPieceVec<S, SS>,
}

impl<S, SS> MosaicArtGenerator<S, SS>
where
    S: Size + MultipleOf<SS>,
    SS: Size + SmallerThan<S>,
{
    pub fn new(
        origin: SizedImage<S>,
        hashtags: HashtagList,
    ) -> (MosaicArtGenerator<S, SS>, MosaicArt<S, SS>) {
        let init_image = SizedImage::clear_image();
        let pieces = MosaicPieceVec::with_origin_image(&origin);
        let distance_f = MeanGrayscale::from_origin(&origin);
        let mut id_gen = IdGenerator::new();

        let init_art = MosaicArt::new(
            id_gen.next_id(),
            init_image.clone(),
            pieces.iter().map(|p| p.post.clone()).collect(),
            hashtags.clone(),
        );
        let generator = MosaicArtGenerator {
            _origin_image: origin,
            hashtags: hashtags.clone(),
            distance_f: distance_f,
            id_gen: IdGenerator::new(),
            current_img: init_image,
            pieces: pieces,
        };

        (generator, init_art)
    }
}

impl<S, SS, D> MosaicArtGenerator<S, SS, D>
where
    S: Size + MultipleOf<SS>,
    SS: Size + SmallerThan<S>,
    D: DistanceFunc<S, SS>,
{
    pub fn hashtags(&self) -> HashtagList {
        self.hashtags.clone()
    }
    pub fn apply_post(&mut self, post: GenericPost<SS>) -> MosaicArt<S, SS> {
        // calc distance between each original image's pieces
        let distance_vec = self.distance_f.distance_vec(&post.image());
        let piece = MosaicPiece {
            post: post,
            distance_vec: distance_vec,
        };
        let (pos, _replaced) = self.pieces.replace_piece(piece.clone());
        self.current_img.overpaint_by(piece.post.image(), pos);

        // Create a new MosaicArt
        let image = self.current_img.clone();
        let posts = self.pieces.iter().map(|piece| piece.post.clone()).collect();
        let hashtags = self.hashtags.clone();
        MosaicArt::new(self.id_gen.next_id(), image, posts, hashtags)
    }

    pub fn has_enough_pieces(&self) -> bool {
        !self.pieces
            .pieces
            .iter()
            .any(|(d, _)| *d == Distance::max_value())
    }
}
