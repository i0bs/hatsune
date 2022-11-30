use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use serenity::model::prelude::command::CommandOptionType;

pub fn run(_options: &[CommandDataOption]) -> String { "Just a test.".to_string() }

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    // /vouch
    command
        .name("vouch")
        .description("Handles vouching for a user in the server.")
        // /vouch info [user]
        .create_option(|option| {
            option
                .name("info")
                .description("Gets information on a vouched user.")
                .kind(CommandOptionType::SubCommand)
                .create_sub_option(|option| {
                    option
                        .name("user")
                        .description("The user to get information on.")
                        .kind(CommandOptionType::User)
                        .required(true)
                })
        })
        // /vouch add [user] [reason?]
        .create_option(|option| {
            option
                .name("add")
                .description("Add a user to the vouched list.")
                .kind(CommandOptionType::SubCommand)
                .create_sub_option(|option| {
                    option
                    .name("user")
                    .description("The user to vouch for.")
                    .kind(CommandOptionType::User)
                    .required(true)
                })
                .create_sub_option(|option| {
                    option
                    .name("reason")
                    .description("The reason why you're vouching for this user.")
                    .kind(CommandOptionType::String)
                    .required(false)
                })
        })
}