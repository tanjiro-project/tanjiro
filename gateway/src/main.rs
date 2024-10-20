use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use twilight_gateway::{Config, Intents, stream::{self}, CloseFrame, Shard, Event};
use twilight_http::Client;
use tokio::signal;
use dotenv::dotenv;

static SHUTDOWN: AtomicBool = AtomicBool::new(false);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN")
        .expect("DISCORD_TOKEN must be set.");

    let client = Client::new(token.clone());
    let config = Config::new(token, Intents::GUILDS | Intents::GUILD_MESSAGES);

    let shards: Vec<_> = stream::create_recommended(&client, config, |_shard, builder| builder.build()).await?.collect();

    let mut senders = Vec::with_capacity(shards.len());
    let mut tasks = Vec::with_capacity(shards.len());

    for shard in shards {
        senders.push(shard.sender());
        tasks.push(tokio::spawn(handle_events(shard)));
    }

    signal::ctrl_c().await?;
    SHUTDOWN.store(true, Ordering::Relaxed);
    for sender in senders {
        // Ignore error if shard's already shutdown.
        sender.close(CloseFrame::NORMAL)?;
    }

    for jh in tasks {
        let _ = jh.await?;
    }

    Ok(())
}

async fn handle_events(mut shard: Shard) -> anyhow::Result<()> {
    tracing::info!("Starting to handle events for shard {:?}", shard.id());

    loop {
        let event = match shard.next_event().await {
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

        match &event {
            Event::GatewayClose(close_event) => {
                tracing::info!(kind = ?event.kind(), shard = ?shard.id(), "received close event: {:?}", close_event);
            },
            Event::MessageCreate(message_event) => {
                tracing::info!(kind = ?event.kind(), shard = ?shard.id(), "received message: {:?}", message_event);
            },
            // Add other event types as needed
            _ => {
                tracing::info!(kind = ?event.kind(), shard = ?shard.id(), "received event of type {:?}", event.kind());
            }
        }
    }

    Ok(())
}