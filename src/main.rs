use teloxide::{
    adaptors::throttle::Limits,
    dispatching::dialogue::{serializer::Json, SqliteStorage},
    prelude::*,
};
use tracing::warn;

mod locale;
mod telegram;

use locale::LocaleManager;
use telegram::MyStorage;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let storage: std::sync::Arc<MyStorage> = SqliteStorage::open("db.sqlite", Json)
        .await
        .expect("Unable to open the dialogue storage.");

    let locale = LocaleManager::new("./res/locale", "en")
        .await
        .expect("Unable to create the locale manager.");

    let bot = Bot::from_env()
        .cache_me()
        .throttle(Limits::default())
        .parse_mode(teloxide::types::ParseMode::Html);

    let handler = telegram::schema();

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![locale, storage])
        .default_handler(|upd| async move {
            warn!("Unhandled update: {:?}", upd);
        })
        .build()
        .dispatch()
        .await;
}
