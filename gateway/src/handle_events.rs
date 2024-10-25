use std::sync::Arc;
use tokio::sync::Mutex;
use twilight_cache_inmemory::InMemoryCache;
use twilight_gateway::Shard;
use twilight_model::gateway::event::Event;
use crate::handlers::message_delete;

pub async fn handle_event(event: Arc<Mutex<Event>>, cache: Arc<Mutex<InMemoryCache>>) {
    let event = event.lock().await; // Lock the mutex to safely access the event

    match &*event {
        Event::MessageDelete(message_event) => {
            message_delete::handle_message_delete_events(message_event, cache).await;
        }
        // Handle other event types as needed
        _ => {
            // info!("Handled event: {:?}", event);
        }
    }
}

pub async fn handle_events(mut shard: Shard, cache: Arc<Mutex<InMemoryCache>>) -> anyhow::Result<()> {
    tracing::info!("Starting to handle events for shard {:?}", shard.id());

    loop {
        let result = match shard.next_event().await {
            Ok(event) => {
                let event_mutex = Arc::new(Mutex::new(event.clone())); // Wrap the event in Arc<Mutex>

                handle_event(event_mutex.clone(), cache.clone()).await;

                // Update the cache with the original event
                let cache_lock = cache.lock().await; // Lock the cache for updating
                cache_lock.update(&event);

                continue; // Continue to the next iteration
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

        // You might want to handle results from your event handling if needed
    }

    Ok(())
}
