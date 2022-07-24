use std::collections::HashMap;
use std::env;
use serenity::async_trait;
use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::Ready;
use serenity::model::prelude::GuildId;
use serenity::prelude::*;
use crate::LeagueAPI;
use crate::services::discord::integrations::Command;

pub struct Handler {
    commands: HashMap<String, Box<dyn Command>>,
}

/// handler for the events of the kurisu client
impl Handler {
    pub fn new() -> Handler {
        return Handler { commands: HashMap::new()};
    }

    pub fn register_slash_command(&mut self, command: Box<dyn Command>) -> &mut Handler {
        self.commands.insert(command.name(), command);
        self
    }

    /// create slash commands for the given guild id
    pub async fn create_slash_commands_for_guild_id(&self, guild_id: GuildId, ctx: Context) {
        GuildId::set_application_commands(&guild_id, &ctx.http, |cmds| {
            for value in self.commands.values().into_iter() {
                cmds.create_application_command(|cmd| {
                    cmd.name(value.name());
                    value.create_command(cmd);
                    cmd
                });
            }
            cmds
        }).await.expect("Failed to load guild commands");
    }
}

#[async_trait]
impl EventHandler for Handler {
    // on ready log information
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Logged in as {}", ready.user.tag());

        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("No guild id supplied, ignoring.")
                .parse()
                .expect("GUILD_ID Must be an integer.")
        );

        self.create_slash_commands_for_guild_id(guild_id, ctx).await;
        println!("Loaded slash commands!");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            if let Some(executor) = self.commands.get(&command.data.name) {
                // log error if it occurs while sending
                executor.respond(&ctx, &command).await;
            }
        }
    }
}

pub async fn create_client(token: &str, handler: Handler, riot_api: LeagueAPI) {
    // create the client
    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(handler)
        .type_map_insert::<LeagueAPI>(riot_api)
        .await
        .expect("Error creating client.");

    // log any errors on client start
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why)
    }
}