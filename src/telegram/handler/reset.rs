use fluent::fluent_args;
use teloxide::prelude::*;

use super::super::{BotType, HandlerResult, MyDialogue};
use crate::locale::LocaleManager;

pub(crate) async fn reset(
    bot: BotType,
    message: Message,
    locale: LocaleManager,
    dialogue: MyDialogue,
) -> HandlerResult {
    let text = locale.get_message("reset", "reset", fluent_args![])?;
    dialogue.exit().await?;
    bot.send_message(message.chat.id, text).await?;

    Ok(())
}
