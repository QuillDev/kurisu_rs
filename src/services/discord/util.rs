use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;

pub async fn send_basic_message(ctx: &Context, event: &ApplicationCommandInteraction, content: &str) {


    if let Err(why) = event.create_interaction_response(&ctx.http, |response| {
        response.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|data| {
                data.ephemeral(true).content(content)
            })
    }).await {
        println!("{:?}", why);
    }
}