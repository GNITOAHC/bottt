use crate::{Context, Error};
use poise::serenity_prelude::Mutex;
use songbird::Call;
use std::future::Future;
use std::sync::Arc;

pub async fn try_join(ctx: Context<'_>, must_join: bool) -> Result<Arc<Mutex<Call>>, &'static str> {
    let (guild, user) = (ctx.guild_id().unwrap(), ctx.author().id);
    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Could not retrieve Songbird voice client")
        .clone();

    if let Some(call) = manager.get(guild) {
        if must_join {
            return Err("Already in a voice channel");
        } else {
            return Ok(call);
        }
    }

    let channel_id = guild
        .to_guild_cached(ctx.serenity_context())
        .unwrap()
        .voice_states
        .get(&user)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => return Err("Not in a voice channel"),
    };

    let (call_handler, _join_result) = manager.join(guild, connect_to).await;

    Ok(call_handler)
}

// voice channel handle
pub async fn vc_handle<
    'a,
    F: FnOnce(Arc<Mutex<Call>>, Context<'a>) -> T + 'a,
    T: Future<Output = Result<(), Error>> + 'a,
>(
    ctx: Context<'a>,
    autojoin: bool,
    f: F,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Could not retrieve Songbird voice client")
        .clone();

    let call_handler = if autojoin {
        match try_join(ctx, false).await {
            Ok(call) => call,
            Err(e) => {
                ctx.say(format!("Failed to autojoin: {e}")).await?;
                return Ok(());
            }
        }
    } else {
        match manager.get(guild_id) {
            // Option<Arc<Mutex<Call>>>
            Some(call) => call,
            None => {
                ctx.say("Not in a voice channel").await?;
                return Ok(());
            }
        }
    };

    // f: FnOnce(Arc<Mutex<Call>>, Context<'a>) -> T + 'a,
    // T: Future<Output = Result<(), Error>> + 'a,
    f(call_handler, ctx).await
}

use songbird::Songbird;
pub async fn leave_handle<
    'a,
    F: FnOnce(Arc<Songbird>, Context<'a>) -> T + 'a,
    T: Future<Output = Result<(), Error>> + 'a,
>(
    ctx: Context<'a>,
    f: F,
) -> Result<(), Error> {
    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Could not retrieve Songbird voice client")
        .clone();
    f(manager, ctx).await
}

#[poise::command(slash_command, prefix_command)]
pub async fn join(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    // vc_handle(ctx, autojoin: bool, f: impl FnOnce(Arc<Mutex<Call>>, Context<'_>) -> Fut) -> Fut
    vc_handle(ctx, true, |_, ctx| async move {
        ctx.say("Joined voice channel").await?;
        Ok(())
    })
    .await
}

#[poise::command(slash_command, prefix_command)]
pub async fn leave(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    leave_handle(ctx, |manager, c| async move {
        let guild_id = c.guild_id().unwrap();
        let has_handler = manager.get(guild_id).is_some();

        if has_handler {
            if let Err(e) = manager.remove(guild_id).await {
                c.say(format!("Failed: {e:?}")).await?;
            } else {
                c.say("Left voice channel").await?;
            }
        } else {
            c.say("Not in a voice channel").await?;
        }

        Ok(())
    })
    .await
}
