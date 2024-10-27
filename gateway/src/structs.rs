use sqlx::{Pool, Postgres};
use twilight_cache_inmemory::InMemoryCache;

pub struct State {
    pub db: Pool<Postgres>,
    pub cache: InMemoryCache
}

impl State {
    pub fn new(db: Pool<Postgres>) -> State {
        Self {
            db,
            cache: InMemoryCache::new()
        }
    }
}
