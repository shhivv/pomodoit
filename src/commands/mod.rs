//! Contains the commands in the bot
//!
//! The `poise::command` macro is used for specifying the commands. All commands are slash commands with owner-only commands being an exception.
//! Slash commands are registered to Discord through the `register` prefix command.

pub mod help;
pub mod register;

pub use help::help;
pub use register::register;
