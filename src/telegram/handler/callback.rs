use teloxide::prelude::*;
use crate::telegram::dialogue::State;

use super::super::{BotType, HandlerResult, MyDialogue};
use crate::locale::LocaleManager;
use fluent::fluent_args;

pub async fn callback_handler(bot: BotType, query: CallbackQuery, dialogue: MyDialogue, locale: LocaleManager) -> HandlerResult {
    // Tell telegram that we've seen this query, to remove 🕑 icons from the clients.
    bot.answer_callback_query(query.id).await?;

    if query.data.is_none() {
        return Ok(());
    }

    match query.data.unwrap().as_str() {
        "employee" => {
            let text = locale.get_message("employee", "select-employer", fluent_args![])?;
            // Edit text of the message to which the buttons were attached
            if let Some(Message { id, chat, .. }) = query.message {
                bot.edit_message_text(chat.id, id, text).await?;
            } else if let Some(id) = query.inline_message_id {
                bot.edit_message_text_inline(id, text).await?;
            }
            dialogue.update(State::EmployeeWaitingEmployer).await?
        },
        "employer" => {
            let text = if let Some(lang) = query.from.language_code {
                dialogue.update(State::EmployerWaitingReport { lang }).await?;
                locale.get_message("employer", "new-employer", fluent_args![])?
            } else {
                dialogue.update(State::EmployerWaitingLang).await?;
                locale.get_message("employer", "new-employer-lang", fluent_args![])?
            };

            if let Some(Message { id, chat, .. }) = query.message {
                bot.edit_message_text(chat.id, id, text).await?;
            } else if let Some(id) = query.inline_message_id {
                bot.edit_message_text_inline(id, text).await?;
            }
        },
        _ => {}
    }

    Ok(())
}