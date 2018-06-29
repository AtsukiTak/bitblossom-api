use std::marker::PhantomData;

use images::{MultipleOf, Size, SizedImage, SmallerThan};

pub type Distance = u64;

pub trait DistanceFunc<S, SS>
where
    S: Size + MultipleOf<SS>,
    SS: Size + SmallerThan<S>,
{
    fn from_origin(origin: &SizedImage<S>) -> Self;
    fn distance_vec(&self, piece: &SizedImage<SS>) -> Vec<Distance>;
}

pub struct MeanGrayscale<S, SS> {
    // Cache of origin piece's mean grayscale
    cache: Vec<f64>,
    _size: PhantomData<(S, SS)>,
}

impl<S, SS> DistanceFunc<S, SS> for MeanGrayscale<S, SS>
where
    S: Size + MultipleOf<SS>,
    SS: Size + SmallerThan<S>,
{
    fn from_origin(origin: &SizedImage<S>) -> MeanGrayscale<S, SS> {
        let cache = origin
            .split_into_pieces()
            .map(|p| p.image.mean_grayscale())
            .collect();
        MeanGrayscale {
            cache: cache,
            _size: PhantomData,
        }
    }

    fn distance_vec(&self, piece: &SizedImage<SS>) -> Vec<Distance> {
        let mean = piece.mean_grayscale();
        self.cache
            .iter()
            .map(move |f| (f64::abs(f - mean) * 10000f64) as u64)
            .collect()
    }
}
