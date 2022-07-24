use serenity::builder::{CreateApplicationCommand};
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction::application_command::CommandDataOptionValue;
use serenity::prelude::Context;
use crate::services::discord::integrations::Command;

use async_trait::async_trait;
use crate::{RiotAPI};
use crate::services::discord::util::send_basic_message;

pub struct Mastery;

#[async_trait]
impl Command for Mastery {
    fn name(&self) -> String {
        return "mastery".to_string();
    }

    fn create_command(&self, cmd: &mut CreateApplicationCommand) {
        cmd.description("Gets your top champion mastery")
            .create_option(|option| {
                option.kind(CommandOptionType::String).name("summoner_name")
                    .required(true)
                    .description("Summoner name to look up")
            });
    }

    async fn respond(&self, ctx: &Context, interaction: &ApplicationCommandInteraction) {
        let name_option = interaction
            .data
            .options.get(0)
            .expect("Expected summoner name!")
            .resolved
            .as_ref()
            .expect("Expected object");

        let mut content: String = "".to_string();
        if let CommandDataOptionValue::String(value) = name_option {
            let mut data = ctx.data.write().await;
            let riot_api = data.get_mut::<RiotAPI>().expect("test");

            if let Ok(mastery) = riot_api.get_mastery_from_name(value).await {
                match mastery.get(0) {
                    None => {}
                    Some(value) => {
                        content = format!("{:?}", value);
                    }
                }
            }
        }

        send_basic_message(ctx, &interaction, content.as_str()).await;
    }
}