//! Pomodoit, currently in development
//!
//! Root file, contains entrypoint to application

#![warn(rust_2018_idioms, clippy::pedantic, missing_docs)]
#![allow(clippy::wildcard_imports)]

mod commands;
pub mod constants;
pub mod core;

use crate::core::connect;

use std::env;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    //Use if bot is not run through docker-compose
    dotenv::dotenv()?;

    tracing_subscriber::fmt::init();
    connect(env::var("DISCORD_TOKEN")?).await?;
    Ok(())
}
