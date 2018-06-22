pub struct MosaicArtGenerator<S, SS> {
    // immutable
    origin_image: Arc<SizedImage<S>>,
    hashtags: Arc<Vec<String>>,
    calc_algo: DistanceCalcAlgo,

    // mutable
    current_art: SizedImage<S>,
    pieces: MosaicPieceVec<S, SS>,
}

impl<S, SS> MosaicArtGenerator<S, SS> {
    pub fn apply_post(&mut self, post: InstaPost<SS>) -> MosaicArt<S, SS> {
        // calc distance between each original image's pieces
        let piece = self.calc_algo.calc(post);
        let (pos, _replaced) = self.pieces.replace_piece(piece.clone());
        self.current_art.overpaint_by(piece.get_image(), pos);
    }
}
