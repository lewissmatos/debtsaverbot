mod file_system;

use std::collections::HashMap;
use std::env;
// use api
use teloxide::{prelude::*, utils::command::BotCommands};

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "Eiron use (-) sign. Lewis use (+) or no sign")]
    Add(String),
    #[command(description = "Calculates new total")]
    Total,
    #[command(description = "Removes the last value")]
    Pop(String),
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting dialogue bot...");
    unsafe {
        env::set_var(
            "TELOXIDE_TOKEN",
            "7952605848:AAEjc7KReYIVJ3c5IrWnrUA0Lj-rzJpavkU",
        );
    }
    let bot = Bot::from_env();
    println!("{}", bot.token());

    Command::repl(bot, answer).await;
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    let user = msg.from.unwrap();
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Add(value) => {
            let val = value.parse::<f64>();

            let msg = match val {
                Ok(val) => {
                    file_system::add_value(val)?;
                    bot.send_message(msg.chat.id, format!("Saved new amount: {}", val))
                }
                Err(err) => {
                    bot.send_message(msg.chat.id, format!("Error saving amiunt\n: {}", err))
                }
            };

            msg.await?
        }
        Command::Total => {
            let total = file_system::get_total()?;
            bot.send_message(msg.chat.id, format!("The total is: {}", total))
                .await?
        }
        Command::Pop(command) => {
            if command.is_empty() || command.to_lowercase() != "confirm" {
                bot.send_message(msg.chat.id, "Invalid param. If you want to remove the last saved value you must type 'confirm' (this includes removing the total if it was the last computed value)".to_string())
                    .await?
            } else {
                let value = file_system::remove_last_value();
                bot.send_message(
                    msg.chat.id,
                    format!(
                        "The value: {} was removed successfully",
                        value.unwrap_or(0.0)
                    ),
                )
                .await?
            }
        }
    };

    Ok(())
}
