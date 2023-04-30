use futures_lite::stream::StreamExt;
use sea_orm::{ConnectOptions, Database};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread::sleep;
use std::time::Duration;
use fluent::fluent_args;
use teloxide::types::{
    InlineKeyboardButton, InlineKeyboardButtonKind, InlineKeyboardMarkup, InputFile,
};
use teloxide::{
    dispatching::dialogue::{serializer::Json, SqliteStorage},
    prelude::*,
};
use tracing::warn;
use uuid::Uuid;

mod locale;
mod telegram;
mod user;

pub use user::ActiveModel as UserActive;
pub use user::Entity as UserEntity;
pub use user::Model as User;

use locale::LocaleManager;
use rabbit::consumer::Consumer;
use rabbit::sender::Sender;
use rabbit::{BasicAckOptions, RabbitClient};
use telegram::MyStorage;

pub const INGOING_MSG_QUEUE: &'static str = "incoming-msgs";
pub const OUTGOING_MSG_QUEUE: &'static str = "outgoing-msgs";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let rabbitmq = RabbitClient::new("amqp://ssh.grassi.dev:5672".to_string())
        .await
        .expect("unable to connect to rabbitmq");

    let sender = Sender::new(
        rabbitmq.clone(),
        "".to_string(),
        INGOING_MSG_QUEUE.to_string(),
    )
    .await
    .expect("unable to create a rabbit sender");

    let consumer = Consumer::new(
        rabbitmq.clone(),
        OUTGOING_MSG_QUEUE.to_string(),
        "".to_string(),
    )
    .await
    .expect("unable to create a rabbitmq consumer");

    let mut opt = ConnectOptions::new("sqlite://db.sqlite".to_string());
    opt.max_connections(2)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .sqlx_logging(true);
    let conn = Database::connect(opt)
        .await
        .expect("unable to connect to the table");

    let storage: Arc<MyStorage> = SqliteStorage::open("dialogue.sqlite", Json)
        .await
        .expect("Unable to open the dialogue storage.");

    let locale = LocaleManager::new("./res/locale", "en")
        .await
        .expect("Unable to create the locale manager.");

    let documents_to_approve: Arc<RwLock<HashMap<Uuid, ModelMessage>>> =
        Arc::new(RwLock::new(HashMap::new()));

    let bot = Bot::from_env()
        .cache_me()
        .parse_mode(teloxide::types::ParseMode::Html);

    let bot_move = bot.clone();
    let documents_to_approve_move = documents_to_approve.clone();
    let locale_move = locale.clone();
    tokio::spawn(async move {
        sleep(std::time::Duration::from_secs(3));
        let mut consumer_channel = consumer
            .channel
            .basic_consume(
                consumer.queue.as_str(),
                consumer.consumer_tag.as_str(),
                consumer.options.clone(),
                consumer.arguments.clone(),
            )
            .await
            .expect("unable to create the consumer channel");

        while let Some(delivery) = consumer_channel.next().await {
            let delivery = delivery.expect("error in consumer");
            let data_string = std::str::from_utf8(delivery.data.as_slice()).unwrap();
            let message: ModelMessage = serde_json::from_str(data_string).unwrap();
            use base64::{engine::general_purpose, Engine as _};
            let pdf_bytes = general_purpose::STANDARD
                .decode(message.clone().pdf.as_str())
                .unwrap();
            let pdf = InputFile::memory(pdf_bytes);
            let pdf = pdf.file_name(format!("{}.pdf", message.title));
            println!("{}", message.employee);
            let employee_id = ChatId(message.employee.parse::<i64>().unwrap());

            let uuid = Uuid::new_v4();
            documents_to_approve_move
                .write()
                .unwrap()
                .insert(uuid, message.clone());


            let text = locale_move.get_message("general", "send", fluent_args![]).unwrap();
            let confirm_button = InlineKeyboardButton::new(
                text,
                InlineKeyboardButtonKind::CallbackData(uuid.to_string()),
            );


            let keyboard = InlineKeyboardMarkup::default().append_row(vec![confirm_button]);
            let text = locale_move.get_message("employee", "summary-generated", fluent_args![
                "reportSummary" => message.report
            ]).unwrap();
            bot_move
                .send_document(employee_id, pdf)
                .caption(text)
                .reply_markup(keyboard)
                .await;

            delivery.ack(BasicAckOptions::default()).await.expect("ack");
        }
    });

    let handler = telegram::schema();

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![
            locale,
            storage,
            sender,
            documents_to_approve,
            conn
        ])
        .default_handler(|upd| async move {
            warn!("Unhandled update: {:?}", upd);
        })
        .build()
        .dispatch()
        .await;
}

#[derive(Deserialize, Debug, Clone)]
pub struct ModelMessage {
    pub employer: String,
    pub employee: String,
    pub title: String,
    pub report: String,
    pub keypoints: String,
    pub translated_title: String,
    pub translated_report: String,
    pub translated_keypoints: String,
    pub images: Vec<String>,
    pub accuracy: String,
    pub pdf: String,
}
