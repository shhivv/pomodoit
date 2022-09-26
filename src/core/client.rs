//! Contains the connect function where the client is built and started

use super::handler::Handler;
use crate::commands::*;

use anyhow::Result;
use poise::serenity_prelude as serenity;
use poise::{Framework, FrameworkOptions};
use std::time::SystemTime;
use tracing::info;

/// Shared data struct
pub struct Data {
    /// Color used for all bot embeds
    pub embed_color: serenity::Color,
    /// Unix timestamp of the bot's starting time
    pub uptime: u64,
}

/// Entrypoint function to connect to the Discord gateway
///
/// # Panics
///
/// Will panic if `SystemTime::now()` returns a value before the Unix epoch
///
/// # Errors
///
/// Will error if any futures fail
pub async fn connect(token: String) -> Result<()> {
    let framework = Framework::builder()
        .options(FrameworkOptions {
            commands: vec![register(), help()],
            on_error: |error| Box::pin(Handler::on_error(error)),
            ..Default::default()
        })
        .token(token)
        .intents(serenity::GatewayIntents::GUILD_MESSAGES | serenity::GatewayIntents::GUILDS)
        .client_settings(|f| f.event_handler(Handler))
        .user_data_setup(move |_, _, _| {
            Box::pin(async move {
                Ok(Data {
                    embed_color: serenity::Colour(16_711_680),
                    uptime: SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                })
            })
        });

    info!("Attempting to connect to Discord gateway");

    framework.run().await?;
    Ok(())
}
