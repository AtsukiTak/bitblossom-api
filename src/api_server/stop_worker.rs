use std::sync::Mutex;
use rocket::{State, response::status::BadRequest};

use worker::{WorkerId, WorkerManager};

#[delete("/worker/<id>")]
fn handler(
    id: u64,
    worker_manager: State<Mutex<WorkerManager>>,
) -> Result<&'static str, BadRequest<()>> {
    if worker_manager
        .inner()
        .lock()
        .unwrap()
        .stop_worker(WorkerId(id))
    {
        Ok("Worker has been stopped")
    } else {
        Err(BadRequest(None))
    }
}
