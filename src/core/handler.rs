//! Contains the `Handler` struct and its impls

use poise::serenity_prelude as serenity;

use serenity::{async_trait, model::gateway::Ready, EventHandler};
use tracing::{error, info};

use super::{Data, Error};

///Holds the bot's event handlers
pub struct Handler;

impl Handler {
    ///Handle any error that is raised within the bot
    ///
    /// # Panics
    ///
    /// Will panic if there is any issue starting the bot
    pub async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
        match error {
            poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot {:?}", error),
            poise::FrameworkError::Command { error, ctx } => {
                error!("Error in command `{}`: {:?}", ctx.command().name, error);
            }
            error => {
                if let Err(e) = poise::builtins::on_error(error).await {
                    println!("Error while handling error: {}", e);
                }
            }
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: serenity::Context, ready: Ready) {
        info!("Successfully logged in as {}", ready.user.tag());
    }
}
