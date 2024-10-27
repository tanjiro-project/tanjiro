use std::sync::Arc;
use sea_query::Query;
use tokio::sync::Mutex;
use twilight_model::channel::Channel;
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
use vesper::context::SlashContext;
use vesper::framework::DefaultCommandResult;
use vesper::macros::command;
use vesper::parsers::TextChannel;
use models::queries::guild_config::{upsert_guild_config, GuildConfigInsertValue};
use crate::structs::State;
use uuid::Uuid;
use chrono::Utc;

#[command]
#[description = "Sets the default logging channel!"]
pub async fn set_default_channel(
    ctx: &SlashContext<'_, Arc<Mutex<State>>>,
    #[description = "The channel will set to"] channel: TextChannel
) -> DefaultCommandResult {
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

    let state = ctx.data.lock().await;
    let guild_id = ctx.interaction.guild_id.unwrap();
    let channel_id = channel.id;

    let insert = upsert_guild_config(&state.db, GuildConfigInsertValue {
        guild_id: guild_id.to_string(),
        default_channel_id: channel_id.to_string()
    }).await;

    match insert {
        Ok(_) => {
            tracing::info!("Updated guild <{}> default logging channel to <{}>", guild_id, channel_id);

            ctx.interaction_client
                .update_response(&ctx.interaction.token)
                .content(Some(&format!("Now default logging channel set to <#{}>", channel_id)))?
                .await?;
        }
        Err(e) => {
            tracing::error!("Failed to insert guild config: {}", e);

            ctx.interaction_client
                .update_response(&ctx.interaction.token)
                .content(Some(&format!("Failed to set the default logging channel. Error: {}", e)))?
                .await?;
        }
    }

    Ok(())
}