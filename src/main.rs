mod commands;

use dotenv;
use serenity::async_trait;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command.data.options),
                "vouch" => commands::vouch::run(&command.data.options),
                _ => "Not implemented.".to_string()
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to interaction: {:?}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is now online.", ready.user.name);

        let guild_id = GuildId(dotenv::var("DISCORD_GUILD_ID").unwrap().parse::<u64>().unwrap());

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::ping::register(command))
                .create_application_command(|command| commands::vouch::register(command))
        })
        .await;

        println!("Guild commands were created: {:#?}", commands);
    }
}

#[tokio::main]
async fn main() {
    let token = dotenv::var("DISCORD_TOKEN").unwrap();

    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Client could not be built");

    if let Err(why) = client.start().await {
        println!("Error with running the client: {:?}", why);
    }
}