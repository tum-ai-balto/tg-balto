use super::super::{BotType, HandlerResult};
use super::Command;

use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;

pub(crate) async fn help(bot: BotType, message: Message) -> HandlerResult {
    bot.send_message(message.chat.id, Command::descriptions().to_string())
        .await?;
    Ok(())
}