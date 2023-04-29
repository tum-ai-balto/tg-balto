use teloxide::utils::command::BotCommands;

pub mod help;
pub mod start;
pub mod callback;

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "verify user.", parse_with = "split")]
    Start,
}
