use serenity::builder::{CreateApplicationCommand, CreateInteractionResponse};
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;

use async_trait::async_trait;

#[async_trait]
pub trait Command : Sync + Send {
    fn name(&self) -> String;
    fn create_command(&self, cmd: &mut CreateApplicationCommand);
    async fn respond(&self, ctx: &Context, interaction: &ApplicationCommandInteraction);
}