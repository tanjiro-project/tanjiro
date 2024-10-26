use std::sync::Arc;
use tokio::sync::Mutex;
use twilight_cache_inmemory::InMemoryCache;
use twilight_gateway::Shard;
use twilight_model::gateway::event::Event;
use crate::handlers::message_delete;

pub async fn handle_event(event: Arc<Mutex<Event>>, cache: Arc<Mutex<InMemoryCache>>) {
    let event = event.lock().await;

    match &*event {
        Event::MessageDelete(message_event) => {
            message_delete::handle_message_delete_events(message_event, cache).await;
        }

        _ => {}
    }
}

pub async fn handle_events(mut shard: Shard, cache: Arc<Mutex<InMemoryCache>>) -> anyhow::Result<()> {
    tracing::info!("Starting to handle events for shard {:?}", shard.id());

    loop {
        let _ = match shard.next_event().await {
            Ok(event) => {
                let event_mutex = Arc::new(Mutex::new(event.clone()));

                handle_event(event_mutex.clone(), cache.clone()).await;

                let cache_lock = cache.lock().await;
                cache_lock.update(&event);

                continue;
            }
            Err(error) if error.is_fatal() => {
                tracing::error!(?error, "fatal error while receiving event");
                break;
            }
            Err(error) => {
                tracing::warn!(?error, "error while receiving event");
                continue;
            }
        };
    }

    Ok(())
}
