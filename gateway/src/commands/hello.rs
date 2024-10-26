use twilight_model::http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType};
use vesper::context::SlashContext;
use vesper::framework::DefaultCommandResult;
use vesper::macros::command;

#[command]
#[description = "Says hello"]
pub async fn hello(ctx: &mut SlashContext<()>) -> DefaultCommandResult {
    ctx.interaction_client.create_response(
        ctx.interaction.id,
        &ctx.interaction.token,
        &InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(InteractionResponseData {
                content: Some(String::from("Hello world")),
                ..Default::default()
            })
        }
    ).await?;

    Ok(())
}
