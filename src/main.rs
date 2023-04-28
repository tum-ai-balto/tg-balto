use teloxide::{adaptors::throttle::Limits, prelude::*};
use tracing::warn;

mod telegram;
mod locale;

use crate::locale::LocaleManager;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    let locale = LocaleManager::new(
        "./res/locale",
        "en",
    )
    .await
    .expect("unable to create the locale manager");

    let bot = Bot::from_env()
        .cache_me()
        .throttle(Limits::default())
        .parse_mode(teloxide::types::ParseMode::Html);

    let handler = telegram::schema();

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![locale])
        .default_handler(|upd| async move {
            warn!("unhandled update: {:?}", upd);
        })
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
