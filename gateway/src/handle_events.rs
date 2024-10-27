use std::sync::Arc;
use std::sync::atomic::Ordering;
use tokio::sync::{Mutex};
use twilight_gateway::Shard;
use twilight_model::gateway::event::Event;
use vesper::framework::Framework;
use crate::handlers::message_delete;
use crate::SHUTDOWN;
use crate::structs::State;

pub async fn handle_events(state: Arc<Mutex<State>>, mut shard: Shard, framework: Arc<Framework<Arc<Mutex<State>>>>) -> anyhow::Result<()> {
    tracing::info!("Starting to handle events for shard {:?}", shard.id());

    let mut locked_state = state.lock().await;

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

        let mut cache = &mut locked_state.cache;

        match &event {
            Event::InteractionCreate(interaction) => {
                cache.update(&event);
                framework.process(interaction.0.clone()).await;
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
