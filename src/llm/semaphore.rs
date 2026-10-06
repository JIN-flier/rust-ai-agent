use std::sync::OnceLock;
use tokio::sync::Semaphore;
static SEMPHORE: OnceLock<Semaphore> = OnceLock::new();

pub fn get_semaphore() -> &'static Semaphore {
    SEMPHORE.get_or_init(|| Semaphore::new(1))
}
