use tracing::{info, warn};
use twilight_cache_inmemory::InMemoryCache;
use twilight_model::gateway::payload::incoming::MessageDelete;

pub(crate) async fn handle_message_delete_events(event: &MessageDelete, cache: &mut InMemoryCache) {
    let message = cache.message(event.id);

    match message {
        Some(msg) => {
            info!(
                "Deleted message {}: content: {:?}",
                event.id,
                msg.content()
            );
        },
        None => {
            warn!("Message {} was not found in cache", event.id);
        }
    }
}