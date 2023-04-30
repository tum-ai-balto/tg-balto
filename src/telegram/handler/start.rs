use super::super::{BotType, HandlerResult};
use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardButtonKind, InlineKeyboardMarkup};

use crate::locale::LocaleManager;
use fluent::fluent_args;

pub(crate) async fn start(bot: BotType, message: Message, locale: LocaleManager) -> HandlerResult {
    let welcome_msg = locale.get_message(
        "start",
        "welcome-message",
        fluent_args![
            "userName" => message.chat.username().unwrap_or("")
        ],
    )?;

    let employer_role = locale.get_message("start", "employer", fluent_args![])?;
    let employee_role = locale.get_message("start", "employee", fluent_args![])?;

    let employer_button = InlineKeyboardButton::new(
        employer_role,
        InlineKeyboardButtonKind::CallbackData("employer".to_string()),
    );
    let employee_button = InlineKeyboardButton::new(
        employee_role,
        InlineKeyboardButtonKind::CallbackData("employee".to_string()),
    );
    let keyboard =
        InlineKeyboardMarkup::default().append_row(vec![employee_button, employer_button]);

    bot.send_message(message.chat.id, welcome_msg)
        .reply_markup(keyboard)
        .await?;

    Ok(())
}
