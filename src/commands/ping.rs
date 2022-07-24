use serenity::builder::{CreateApplicationCommand, CreateInteractionResponse};
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::prelude::Context;
use crate::services::discord::integrations::Command;

pub struct Ping;

use async_trait::async_trait;

#[async_trait]
impl Command for Ping {
    fn name(&self) -> String {
        return "ping".to_string();
    }

    fn create_command(&self, cmd: &mut CreateApplicationCommand) {
        cmd.description("testing mod v2");
    }

    async fn respond(&self, ctx: &Context, interaction: &ApplicationCommandInteraction){
        interaction.create_interaction_response(&ctx.http, |res| {
            res.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|data| {
                    data
                        .ephemeral(true)
                        .content("uhhh?")
                })
        }).await;
    }
}