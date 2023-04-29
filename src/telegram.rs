use std::sync::Arc;
use teloxide::dptree::case;
use teloxide::{
    adaptors::{CacheMe, DefaultParseMode, Throttle},
    dispatching::{dialogue as dialogue1, dialogue::serializer::Json, dialogue::SqliteStorage, UpdateHandler},
    prelude::*,
};

mod handler;
mod dialogue;

use crate::locale::LocaleManager;
use handler::Command;
use dialogue::State;

pub type BotType = DefaultParseMode<Throttle<CacheMe<Bot>>>;
pub type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
pub type MyStorage = SqliteStorage<Json>;
pub type MyDialogue = Dialogue<State, MyStorage>;

pub(crate) fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(case![State::Start]
            .branch(case![Command::Start].endpoint(handler::start::start)))
        .branch(case![Command::Help].endpoint(handler::help::help))
        .branch();

    let message_handler = Update::filter_message()
        .map(|message: Message, mut locale: LocaleManager| {
            locale.set_chat_locale_from_message(&message);
            locale
        })
        .branch(case![State::EmployeeWaitingEmployer].endpoint(dialogue::employee::waiting_employer)) // TODO
        .branch(case![State::EmployerWaitingLang].endpoint(handler::help::help)) // TODO
        .branch(command_handler);

    let callback_query_handler = Update::filter_callback_query()
        .map(|query: CallbackQuery, mut locale: LocaleManager| {
            locale.set_chat_locale_from_query(&query);
            locale
        })
        .branch(case![State::Start].endpoint(handler::callback::callback_handler));

    dptree::entry().enter_dialogue::<Update, MyStorage, State>()
        .branch(message_handler)
        .branch(callback_query_handler)
}





