use std::sync::Arc;
use std::sync::atomic::Ordering;
use tokio::sync::{Mutex};
use twilight_cache_inmemory::InMemoryCache;
use twilight_gateway::Shard;
use twilight_http::Client;
use twilight_model::gateway::event::Event;
use twilight_model::id::Id;
use twilight_model::id::marker::ApplicationMarker;
use vesper::framework::Framework;
use crate::handlers::message_delete;
use crate::commands::hello::hello;
use crate::SHUTDOWN;

pub async fn handle_events(http_client: Arc<Client>, app_id: Id<ApplicationMarker>, mut shard: Shard, cache: Arc<Mutex<InMemoryCache>>) -> anyhow::Result<()> {
    tracing::info!("Starting to handle events for shard {:?}", shard.id());

    let framework = Arc::new(Framework::builder(http_client, app_id, ())
        .command(hello)
        .build());

    framework.register_global_commands().await?;

    loop {
        let event = match shard.next_event().await {
            Ok(Event::GatewayClose(close_event)) => {
                tracing::info!(shard = ?shard.id(), "received close event: {:?}", close_event);

                if SHUTDOWN.load(Ordering::Relaxed) {
                    break;
                }

                continue;
            },
            Ok(event) => event,
            Err(error) if error.is_fatal() => {
                tracing::error!(?error, "fatal error while receiving event");
                break;
            }
            Err(error) => {
                tracing::warn!(?error, "error while receiving event");
                continue;
            }
        };

        let mut cache = cache.lock().await;

        match &event {
            Event::InteractionCreate(interaction) => {
                cache.update(&event);
                let framework_clone = Arc::clone(&framework);
                framework_clone.process(interaction.0.clone()).await;
            },

            Event::MessageDelete(message) => {
                message_delete::handle_message_delete_events(message, &mut cache).await;
                cache.update(&event);
            }

            _ => {
                cache.update(&event);
                tracing::info!(kind = ?event.kind(), shard = ?shard.id(), "received event of type {:?}", event.kind());
            }
        }
    }

    Ok(())
}
