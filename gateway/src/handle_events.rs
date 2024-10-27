use std::sync::Arc;
use tokio::sync::{Mutex};
use twilight_model::gateway::event::Event;
use vesper::framework::Framework;
use crate::structs::State;

pub async fn handle_events(state: Arc<Mutex<State>>, event: Event, framework: Arc<Framework<Arc<Mutex<State>>>>) -> anyhow::Result<()> {
    let state_guard = state.lock().await;

    let ev = event.clone();

    match event {
        Event::InteractionCreate(interaction) => {
            state_guard.cache.update(&ev);
            tokio::spawn(async move {
                let inner = interaction.0;
                framework.process(inner).await;
            });
        },

        _ => {
            state_guard.cache.update(&ev);
            tracing::info!(kind = ?event.kind(), "received event of type {:?}", event.kind());
        }
    }

    Ok(())
}
