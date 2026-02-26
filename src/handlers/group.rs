use teloxide::prelude::*;
use crate::BotState;

pub async fn handle_group_message(
    bot: Bot,
    msg: Message,
    state: BotState,
) -> ResponseResult<()> {
    let username = msg.from()
        .and_then(|u| u.username.clone());

    if let Some(username) = username {
        let mappings = state.response_mappings.lock().await;
        if let Some(response) = mappings.get(&username) {
            bot.send_message(msg.chat.id, response).await?;
            log::info!("Sent response to {} in group {}", username, msg.chat.id);
        }
    }

    Ok(())
}
