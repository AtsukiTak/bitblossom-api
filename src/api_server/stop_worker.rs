use std::sync::Mutex;
use rocket::{State, response::status::BadRequest};

use worker::{WorkerId, WorkerManager};
use super::{OriginImageSize, PieceImageSize};

#[delete("/worker/<id>")]
fn handler(
    id: u64,
    worker_manager: State<Mutex<WorkerManager<OriginImageSize, PieceImageSize>>>,
) -> Result<&'static str, BadRequest<()>> {
    if worker_manager
        .inner()
        .lock()
        .unwrap()
        .stop_worker(WorkerId::from_raw(id))
    {
        Ok("Worker has been stopped")
    } else {
        Err(BadRequest(None))
    }
}
