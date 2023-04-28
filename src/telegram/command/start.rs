use super::super::{BotType, HandlerResult};
use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, InlineKeyboardButtonKind};

use fluent::fluent_args;
use crate::locale::LocaleManager;

pub(crate) async fn start(bot: BotType, message: Message, locale: LocaleManager) -> HandlerResult {
    let employer_role = locale.get_message("start", "employer", fluent_args![])?;
    let employee_role = locale.get_message("start", "employee", fluent_args![])?;
    let what_is_role = locale.get_message("start", "what-is-role", fluent_args![])?;

    let employer_button = InlineKeyboardButton::new(employer_role, InlineKeyboardButtonKind::CallbackData("employer".to_string()));
    let employee_button = InlineKeyboardButton::new(employee_role, InlineKeyboardButtonKind::CallbackData("employee".to_string()));
    let keyboard = InlineKeyboardMarkup::default().append_row(vec![employee_button, employer_button]);

    bot.send_message(message.chat.id, what_is_role)
        .reply_markup(keyboard)
        .await?;

    Ok(())
}