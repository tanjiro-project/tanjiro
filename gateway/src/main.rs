use std::env;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use twilight_gateway::{Config, Intents, stream::{self}, CloseFrame};
use twilight_http::Client;
use tokio::signal;
use dotenv::dotenv;
use sqlx::PgPool;
use tokio::sync::{Mutex};
use twilight_gateway::stream::ShardEventStream;
use twilight_model::gateway::payload::outgoing::update_presence::UpdatePresencePayload;
use twilight_model::gateway::presence::{ActivityType, MinimalActivity, Status};
use twilight_model::id::Id;
use twilight_model::id::marker::ApplicationMarker;
use vesper::framework::Framework;
use crate::handle_events::{handle_events};
use crate::structs::State;

use futures::StreamExt;

mod handlers;
mod handle_events;
mod commands;
mod structs;

static SHUTDOWN: AtomicBool = AtomicBool::new(false);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    tracing_subscriber::fmt::init();

    let version = env!("CARGO_PKG_VERSION");

    let token = env::var("DISCORD_TOKEN")
        .expect("DISCORD_TOKEN must be set.");

    let app_id = Id::<ApplicationMarker>::new(std::env::var("APP_ID")?.parse()?);
    let client = Arc::new(Client::new(token.clone()));

    let config = Config::builder(token.clone(), Intents::GUILDS | Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT)
        .presence(UpdatePresencePayload::new(
            vec![MinimalActivity {
                kind: ActivityType::Listening,
                name: format!("Logging | v{}", version),
                url: None,
            }
                .into()],
            false,
            None,
            Status::Online,
        )?)
        .build();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = PgPool::connect(&database_url).await?;

    let mut shards: Vec<_> = stream::create_recommended(&client, config, |_shard, builder| builder.build())
        .await?
        .collect();
    let mut shard_stream = ShardEventStream::new(shards.iter_mut());
    let state = Arc::new(Mutex::new(State::new(db)));

    let framework = Arc::new(Framework::builder(Arc::clone(&client), app_id, Arc::clone(&state))
        .command(commands::ping::ping)
        .group(|g| {
            g.name("settings")
                .description("Manage Bot Settings")
                .command(commands::settings::set_default_channel::set_default_channel)
        })
        .build());

    framework.register_global_commands().await?;

    while let Some(event) = shard_stream.next().await {
        let ev = event.1?;

        let res = handle_events(state.clone(), ev, Arc::clone(&framework)).await;
        if let Err(res) = res {
            tracing::error!("{:?}", res);
        }
    }

    Ok(())
}
