use image::DynamicImage;

pub trait PieceImage {
    const WIDTH: u32;
    const HEIGHT: u32;

    fn image(&self) -> &DynamicImage;

    fn mean_grayscale(&self) -> f64 {
        super::mean_grayscale(&self.image().to_luma())
    }
}

pub struct MiniPieceImage {
    image: DynamicImage,
}

impl PieceImage for MiniPieceImage {
    const WIDTH:u32 = 30;
    const HEIGHT:u32 = 30;

    fn image(&self) -> &DynamicImage {
        &self.image
    }
}
