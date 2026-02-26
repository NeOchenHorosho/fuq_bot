use teloxide::prelude::*;
use crate::BotState;

pub async fn handle_private_message(
    bot: Bot,
    msg: Message,
    state: BotState,
) -> ResponseResult<()> {
    let text = msg.text().unwrap_or("");
    let username = msg.from()
        .and_then(|u| u.username.clone());

    if username.is_none() {
        bot.send_message(msg.chat.id, "Error: No username found.").await?;
        return Ok(());
    }

    let username = username.unwrap();
    let whitelist = state.whitelist.lock().await;
    let is_whitelisted = whitelist.contains(&username);
    drop(whitelist);

    if !is_whitelisted {
        bot.send_message(msg.chat.id, "You are not authorized to configure this bot.").await?;
        return Ok(());
    }

    let parts: Vec<&str> = text.split_whitespace().collect();
    
    if parts.is_empty() {
        bot.send_message(msg.chat.id, "Use /help to see available commands.").await?;
        return Ok(());
    }

    match parts[0] {
        "/help" => {
            let help_text = r#"Available commands:
/adduser <username> <message> - Add/update a user response
/removeuser <username> - Remove a user mapping
/list - List all configured mappings
/whitelist add <username> - Add user to whitelist
/whitelist remove <username> - Remove from whitelist
/whitelist list - List whitelist"#;
            bot.send_message(msg.chat.id, help_text).await?;
        }
        "/adduser" => {
            if parts.len() < 3 {
                bot.send_message(msg.chat.id, "Usage: /adduser <username> <message>").await?;
                return Ok(());
            }
            let target_username = parts[1].trim_start_matches('@');
            let response_msg = parts[2..].join(" ");
            let mut mappings = state.response_mappings.lock().await;
            mappings.insert(target_username.to_string(), response_msg.clone());
            bot.send_message(msg.chat.id, format!("Added: @{} -> {}", target_username, response_msg)).await?;
        }
        "/removeuser" => {
            if parts.len() < 2 {
                bot.send_message(msg.chat.id, "Usage: /removeuser <username>").await?;
                return Ok(());
            }
            let target_username = parts[1].trim_start_matches('@');
            let mut mappings = state.response_mappings.lock().await;
            if mappings.remove(target_username).is_some() {
                bot.send_message(msg.chat.id, format!("Removed mapping for @{}", target_username)).await?;
            } else {
                bot.send_message(msg.chat.id, format!("No mapping found for @{}", target_username)).await?;
            }
        }
        "/list" => {
            let mappings = state.response_mappings.lock().await;
            if mappings.is_empty() {
                bot.send_message(msg.chat.id, "No mappings configured.").await?;
            } else {
                let list: Vec<String> = mappings
                    .iter()
                    .map(|(k, v)| format!("@{} -> {}", k, v))
                    .collect();
                bot.send_message(msg.chat.id, list.join("\n")).await?;
            }
        }
        "/whitelist" => {
            if parts.len() < 2 {
                bot.send_message(msg.chat.id, "Usage: /whitelist <add|remove|list> [username]").await?;
                return Ok(());
            }
            match parts[1] {
                "add" => {
                    if parts.len() < 3 {
                        bot.send_message(msg.chat.id, "Usage: /whitelist add <username>").await?;
                        return Ok(());
                    }
                    let target = parts[2].trim_start_matches('@');
                    let mut whitelist = state.whitelist.lock().await;
                    if !whitelist.contains(&target.to_string()) {
                        whitelist.push(target.to_string());
                    }
                    bot.send_message(msg.chat.id, format!("Added @{} to whitelist", target)).await?;
                }
                "remove" => {
                    if parts.len() < 3 {
                        bot.send_message(msg.chat.id, "Usage: /whitelist remove <username>").await?;
                        return Ok(());
                    }
                    let target = parts[2].trim_start_matches('@');
                    let mut whitelist = state.whitelist.lock().await;
                    whitelist.retain(|u| u != target);
                    bot.send_message(msg.chat.id, format!("Removed @{} from whitelist", target)).await?;
                }
                "list" => {
                    let whitelist = state.whitelist.lock().await;
                    if whitelist.is_empty() {
                        bot.send_message(msg.chat.id, "Whitelist is empty.").await?;
                    } else {
                        bot.send_message(msg.chat.id, format!("Whitelist: @{}", whitelist.join(", @"))).await?;
                    }
                }
                _ => {
                    bot.send_message(msg.chat.id, "Unknown /whitelist command. Use: add, remove, or list").await?;
                }
            }
        }
        _ => {
            bot.send_message(msg.chat.id, "Unknown command. Use /help for available commands.").await?;
        }
    }

    Ok(())
}
