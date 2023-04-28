use teloxide::prelude::*;

#[tokio::main]
async fn main() {

    pretty_env_logger::init();

    let bot = Bot::from_env();

    teloxide::repl(bot, |_bot: Bot, _msg: Message| async move {
        Ok(())
    })
    .await;
}