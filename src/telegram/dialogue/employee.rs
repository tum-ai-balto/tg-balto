use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardButtonKind, InlineKeyboardMarkup};
use fluent::fluent_args;

use crate::telegram::dialogue::State;
use crate::locale::LocaleManager;
use super::super::{BotType, HandlerResult, MyDialogue};

pub(crate) async fn waiting_employer(bot: BotType, message: Message, locale: LocaleManager, dialogue: MyDialogue) -> HandlerResult {
    let text = locale.get_message("employee", "received-employer", fluent_args![])?;

    bot.send_message(message.chat.id, text)
        .await?;

    dialogue.update(State::EmployeeWaitingMedia { employer: message.text().unwrap().to_string(), content: vec![] }).await?;
    Ok(())
}