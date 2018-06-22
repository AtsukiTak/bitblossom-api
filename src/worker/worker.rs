pub struct WorkerManager {
    insta_feeder: Arc<InstaFeeder>,
    db: Mongodb,
}

impl WorkerManager {
    pub fn start_worker(&self, hashtags: Vec<String>, origin: SizedImage<S>) -> Worker {
    }
}

pub struct Worker {
    art: Mutex<MosaicArt>,
}
