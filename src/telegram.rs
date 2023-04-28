use teloxide::dispatching::UpdateHandler;
use teloxide::dptree::case;
use teloxide::{
    adaptors::{CacheMe, DefaultParseMode, Throttle},
    prelude::*,
};

mod command;

use command::Command;
use crate::locale::LocaleManager;

type BotType = DefaultParseMode<Throttle<CacheMe<Bot>>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub(crate) fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(case![Command::Start].endpoint(command::start::start))
        .branch(case![Command::Help].endpoint(command::help::help));

    let message_handler = Update::filter_message()
        .map(|message: Message, mut locale: LocaleManager| {
            locale.set_chat_locale_from_message(&message);
            locale
        })
        .branch(command_handler);

    dptree::entry().branch(message_handler)
}