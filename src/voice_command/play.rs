use crate::voice_command::vc_handle;
use crate::{Context, Error};
use songbird::ytdl;

#[poise::command(slash_command)]
pub async fn play(ctx: Context<'_>, #[description = "url"] url: String) -> Result<(), Error> {
    ctx.defer().await?;
    // vc_handle(ctx, autojoin: bool, f: impl FnOnce(Arc<Mutex<Call>>, Context<'_>) -> Fut) -> Fut
    vc_handle(ctx, true, |call, c| async move {
        let mut call = call.lock().await; // MutexGuard<Call>
        let source = match ytdl(url).await {
            Ok(source) => source,
            Err(e) => {
                c.say(format!("Failed to play: {e}")).await?;
                return Ok(());
            }
        };
        let title = source.metadata.title.clone();
        call.enqueue_source(source);
        match title {
            Some(title) => {
                c.say(format!("Adding {} to queue", title)).await?;
                return Ok(());
            }
            None => {
                c.say("Failed to enqueue").await?;
                return Ok(());
            }
        }
    })
    .await
}

#[poise::command(slash_command)]
pub async fn pause(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    vc_handle(ctx, false, |call, c| async move {
        let call = call.lock().await;
        let _ = call.queue().pause();
        c.say("pause").await?;
        Ok(())
    })
    .await
}

#[poise::command(slash_command)]
pub async fn resume(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    vc_handle(ctx, false, |call, c| async move {
        let call = call.lock().await;
        let _ = call.queue().resume();
        c.say("resume").await?;
        Ok(())
    })
    .await
}

#[poise::command(slash_command)]
pub async fn stop(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    vc_handle(ctx, false, |call, c| async move {
        let call = call.lock().await;
        let _ = call.queue().stop();
        c.say("stop").await?;
        Ok(())
    })
    .await
}

#[poise::command(slash_command)]
pub async fn skip(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    vc_handle(ctx, false, |call, c| async move {
        let call = call.lock().await;
        let _ = call.queue().skip();
        c.say("skip").await?;
        Ok(())
    })
    .await
}
