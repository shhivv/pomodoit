//! Contains the core client application and related utilities

pub mod client;
pub mod handler;

pub use client::*;

///Represents an error raised in the bot
pub type Error = Box<dyn std::error::Error + Send + Sync>;

///Represents the context object received in commands
pub type Context<'a> = poise::Context<'a, Data, Error>;
