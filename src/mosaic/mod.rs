pub mod insta_api;
pub mod position_finder;
pub mod worker;
pub mod container;

pub use self::insta_api::{InstaApi, InstaPartialPost, InstaPost, InstaPostId};
pub use self::position_finder::GrayscalePositionFinder;
pub use self::worker::Worker;
pub use self::container::ArtContainer;

use std::{collections::HashMap, marker::PhantomData};
use image::{Rgba, RgbaImage};
use images::{Image, Position, size::{Size, SmallerThan}};

pub struct MosaicArtImage<S> {
    image: RgbaImage,
    phantom: PhantomData<S>,
}

impl<S: Size> MosaicArtImage<S> {
    /// Create a clear MosaicArt.
    pub fn new() -> MosaicArtImage<S> {
        const CLEAR_PIXEL: Rgba<u8> = Rgba { data: [0, 0, 0, 0] };
        let clear_img = RgbaImage::from_pixel(S::WIDTH, S::HEIGHT, CLEAR_PIXEL);
        MosaicArtImage {
            image: clear_img,
            phantom: PhantomData,
        }
    }

    pub fn with_base<I>(base_img: I) -> MosaicArtImage<S>
    where
        I: Image<Size = S, Image = RgbaImage, Pixel = Rgba<u8>>,
    {
        MosaicArtImage {
            image: base_img.image().clone(),
            phantom: PhantomData,
        }
    }
}

impl<S: Size> Image for MosaicArtImage<S> {
    type Size = S;
    type Image = RgbaImage;
    type Pixel = Rgba<u8>;

    fn image(&self) -> &RgbaImage {
        &self.image
    }

    fn image_mut(&mut self) -> &mut RgbaImage {
        &mut self.image
    }
}

pub struct MosaicArt<S> {
    image: MosaicArtImage<S>,
    piece_posts: HashMap<InstaPostId, InstaPost>,
    position_map: HashMap<Position, InstaPostId>,
    hashtags: Vec<String>,
}

impl<S: Size> MosaicArt<S> {
    pub fn new(hashtags: Vec<String>) -> MosaicArt<S> {
        MosaicArt {
            image: MosaicArtImage::new(),
            piece_posts: HashMap::new(),
            position_map: HashMap::new(),
            hashtags: hashtags,
        }
    }

    pub fn with_base<I>(base_img: I, hashtags: Vec<String>) -> MosaicArt<S>
    where
        I: Image<Size = S, Image = RgbaImage, Pixel = Rgba<u8>>,
    {
        MosaicArt {
            image: MosaicArtImage::with_base(base_img),
            piece_posts: HashMap::new(),
            position_map: HashMap::new(),
            hashtags: hashtags,
        }
    }
    pub fn has_post(&self, post_id: &InstaPostId) -> bool {
        self.piece_posts.contains_key(post_id)
    }

    pub fn apply_new_image<I>(&mut self, image: I, post: InstaPost, pos: Position)
    where
        I: Image<Pixel = Rgba<u8>>,
        I::Size: SmallerThan<S>,
    {
        self.image.overpaint_by(image, pos.clone());
        if let Some(overrided_post_id) = self.position_map.insert(pos, post.post_id.clone()) {
            self.piece_posts.remove(&overrided_post_id);
        }
        self.piece_posts.insert(post.post_id.clone(), post);
    }

    pub fn get_image(&self) -> &MosaicArtImage<S> {
        &self.image
    }

    pub fn get_hashtags(&self) -> &Vec<String> {
        &self.hashtags
    }

    pub fn get_piece_posts(&self) -> impl Iterator<Item = &InstaPost> {
        self.piece_posts.values()
    }
}
