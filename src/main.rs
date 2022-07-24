extern crate core;

mod services;
mod commands;

use std::env;
use std::error::Error;
use crate::commands::mastery::Mastery;
use crate::commands::ping::Ping;
use crate::services::discord::client::{create_client, Handler};
use crate::services::riot::league_api::LeagueAPI;
use crate::services::util::downloads::download_file;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    // get the riot api key
    let reqwest = reqwest::Client::new();
    let riot_api_key = env::var("RIOT_API_KEY").expect("Expected riot API Key.");
    let discord_token = env::var("TOKEN").expect("Expected a token secret to be supplied.");

    let mut league_api = LeagueAPI::new(reqwest, riot_api_key);

    let res= league_api.get_latest_dd_data().await;
    match res {
        Ok(ok) => {
            println!("{}", ok)
        }
        Err(err) => {
            println!("Error occured downloading data: {:?}", err);
            return Err(panic!("Could not download data!"));
        }
    }

    let mut handler = Handler::new();
    handler
        .register_slash_command(Box::new(Ping))
        .register_slash_command(Box::new(Mastery));

    create_client(discord_token.as_str(), handler, league_api).await;
    Ok(())
}