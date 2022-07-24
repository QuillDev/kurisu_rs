extern crate core;

mod services;
mod commands;

use std::env;
use std::error::Error;
use crate::commands::mastery::Mastery;
use crate::commands::ping::Ping;
use crate::riot_api::RiotAPI;
use crate::services::discord::client::{create_client, Handler};
use crate::services::riot::riot_api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    // get the riot api key
    let reqwest = reqwest::Client::new();
    let riot_api_key = env::var("RIOT_API_KEY").expect("Expected riot API Key.");
    let discord_token = env::var("TOKEN").expect("Expected a token secret to be supplied.");

    let riot_api = RiotAPI::new(reqwest, riot_api_key);
    let mut handler = Handler::new();
    handler
        .register_slash_command(Box::new(Ping))
        .register_slash_command(Box::new(Mastery));

    create_client(discord_token.as_str(), handler, riot_api).await;
    Ok(())
}