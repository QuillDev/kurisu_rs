use serenity::builder::{CreateApplicationCommand};
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction::application_command::CommandDataOptionValue;
use serenity::prelude::Context;
use crate::services::discord::integrations::Command;

use async_trait::async_trait;
use reqwest::Error;
use crate::LeagueAPI;
use crate::services::discord::util::send_basic_message;
use crate::services::riot::models::ChampionMastery;

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
            .expect("Expected name option");

        if let CommandDataOptionValue::String(value) = name_option {

            // grab the league api
            let mut data = ctx.data.write().await;
            let league_api = data
                .get_mut::<LeagueAPI>()
                .expect("could not get league api");

            // get mastery from the given name
            match league_api.get_mastery_from_name(value).await {
                // if the data is valid
                Ok(data) => {
                    match data.get(0) {
                        Some(value) => send_basic_message(ctx, &interaction, format!("{:?}", value).as_str()).await,
                        None => send_basic_message(ctx, &interaction, "Could not find champion data.").await,
                    }
                }
                // if the data is invalid
                Err(err) => {
                    send_basic_message(ctx, &interaction, "An error occurred.");
                    println!("{:?}", err);
                }
            }
        }
    }
}