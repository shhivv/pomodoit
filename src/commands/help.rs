use super::super::constants::*;
use crate::core::{Context, Error};

use poise::serenity_prelude as serenity;
use std::time::SystemTime;

///🛠️ Help on using the bot
#[poise::command(slash_command)]
pub async fn help(ctx: Context<'_>) -> Result<(), Error> {
    let initial = SystemTime::now();

    let reply = ctx
        .say(format!("{} Calculating latency", LOADING_EMOJI))
        .await?;

    let elapsed = initial.elapsed()?.as_millis();
    let cache = &ctx.discord().cache;
    let data = ctx.data();

    reply.edit(ctx,|m| {
        m.content("").embed(|e| {
            e.color(data.embed_color)
                .title("`🚀️` Pomodoit Help")
                .description(format!(
                    r#"
                    Pomodoit brings the famous Pomodoro technique to Discord, so that you can 
                    collaborate and focus at the same time!

                    Use the `(/)start` command to get started!
                    
                    > [Learn more](https://en.wikipedia.org/wiki/Pomodoro_Technique) about the pomodoro technique.
                    > Currently helping `{}` servers and `{}` members focus!
                    > The bot has been online since <t:{}:R> and has a latency of `{}ms`
            "#, cache.guild_count(), cache.user_count(), data.uptime, elapsed),
                )
                .footer(|f| {
                    f.text("Built by shiv#6819 with Serenity 🦀")
                        .icon_url("https://avatars.githubusercontent.com/u/75499121?v=4")
                })
        }).components(|f| f.create_action_row(
                            |f| f.create_button(
                                |f| f.style(serenity::ButtonStyle::Link).url(REPO_URL).label("🦄 View source code")
                        )
                    )
                )
    })
    .await?;
    Ok(())
}
