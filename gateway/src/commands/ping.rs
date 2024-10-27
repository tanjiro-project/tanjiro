use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::Instant;
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
use vesper::context::SlashContext;
use vesper::framework::DefaultCommandResult;
use vesper::macros::command;
use crate::structs::State;

#[command]
#[description = "Ping Pong with the bot!"]
pub async fn ping(ctx: &SlashContext<'_, Arc<Mutex<State>>>) -> DefaultCommandResult {
    let start = Instant::now();

    ctx.interaction_client
        .create_response(
            ctx.interaction.id,
            &ctx.interaction.token,
            &InteractionResponse {
                kind: InteractionResponseType::DeferredChannelMessageWithSource,
                data: None,
            },
        )
        .await?;

    let latency = start.elapsed().as_millis();

    ctx.interaction_client
        .update_response(&ctx.interaction.token)
        .content(Some(&format!("🏓 | Pong! Latency: {} ms", latency)))?
        .await?;

    Ok(())
}
