use teloxide::{
    adaptors::{CacheMe, DefaultParseMode},
    dispatching::{dialogue::serializer::Json, dialogue::SqliteStorage, UpdateHandler},
    dptree::case,
    prelude::*,
    types::Message,
};

mod handler;

use crate::locale::LocaleManager;
use handler::Command;
use handler::State;

pub type BotType = DefaultParseMode<CacheMe<Bot>>;
pub type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
pub type MyStorage = SqliteStorage<Json>;
pub type MyDialogue = Dialogue<State, MyStorage>;

pub(crate) fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(case![State::Start].branch(case![Command::Start].endpoint(handler::start::start)))
        .branch(case![Command::Help].endpoint(handler::help::help))
        .branch(case![Command::Reset].endpoint(handler::reset::reset))
        .branch(case![Command::Employers].endpoint(handler::employee::get_employers_list))
        .branch(
            case![Command::Send].chain(
                case![State::EmployeeWaitingMedia { employer, medias }]
                    .endpoint(handler::employee::send_media),
            ),
        ); // todo!("invalid command")

    let message_handler = Update::filter_message()
        .map(|message: Message, mut locale: LocaleManager| {
            locale.set_chat_locale_from_message(&message);
            locale
        })
        .branch(command_handler)
        .branch(case![State::Start].endpoint(handler::start::start))
        .branch(
            Message::filter_text()
                .branch(
                    case![State::EmployeeWaitingEmployer]
                        .endpoint(handler::employee::waiting_employer),
                )
                .branch(
                    case![State::EmployeeWaitingMedia { employer, medias }]
                        .endpoint(handler::employee::waiting_text),
                )
                .branch(case![State::EmployerWaitingLang].endpoint(handler::help::help)), // TODO
        )
        .branch(
            case![State::EmployeeWaitingMedia { employer, medias }]
                .branch(Message::filter_photo().endpoint(handler::employee::waiting_photo))
                .branch(
                    dptree::filter(|message: Message| message.voice().is_some())
                        .endpoint(handler::employee::waiting_voice),
                ),
        );

    let callback_query_handler = Update::filter_callback_query()
        .map(|query: CallbackQuery, mut locale: LocaleManager| {
            locale.set_chat_locale_from_query(&query);
            locale
        })
        .branch(case![State::Start].endpoint(handler::callback::callback_handler))
        .branch(
            case![State::EmployeePendingReport]
                .endpoint(handler::callback::callback_document_handler),
        );

    dptree::entry()
        .enter_dialogue::<Update, MyStorage, State>()
        .branch(message_handler)
        .branch(callback_query_handler)
}
