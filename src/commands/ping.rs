use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;
use crate::services::discord::integrations::Command;

pub struct Ping;

use async_trait::async_trait;
use serenity::builder::CreateApplicationCommand;
use crate::services::discord::util::send_basic_message;

#[async_trait]
impl Command for Ping {
    fn name(&self) -> String {
        return "ping".to_string();
    }

    fn create_command(&self, cmd: &mut CreateApplicationCommand) {
        cmd.description("testing mod v2");
    }

    async fn respond(&self, ctx: &Context, event: &ApplicationCommandInteraction){
        send_basic_message(&ctx, &event, "Pong!").await;
    }
}