use std::collections::HashMap;
use std::sync::Arc;
use teloxide::prelude::*;
use teloxide::types::Message;
use tokio::sync::Mutex;

mod handlers;

#[derive(Clone)]
pub struct BotState {
    pub whitelist: Arc<Mutex<Vec<String>>>,
    pub response_mappings: Arc<Mutex<HashMap<String, String>>>,
}

impl BotState {
    fn new() -> Self {
        Self {
            whitelist: Arc::new(Mutex::new(Vec::new())),
            response_mappings: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    log::info!("Starting fuq_bot...");

    let bot = Bot::from_env();
    let state = BotState::new();

    let handler = Update::filter_message()
        .branch(
            dptree::entry()
                .filter(|msg: Message| msg.chat.is_group() || msg.chat.is_supergroup())
                .endpoint(handlers::handle_group_message),
        )
        .branch(
            dptree::entry()
                .filter(|msg: Message| msg.chat.is_private())
                .endpoint(handlers::handle_private_message),
        );

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![state])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
