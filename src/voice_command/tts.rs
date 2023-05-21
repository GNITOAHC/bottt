use crate::voice_command::vc_handle;
use crate::{Context, Error};
use songbird::ytdl;

async fn language_autocomplete<'a>(
    _ctx: Context<'_>,
    _partial: &'a str,
) -> impl Iterator<Item = poise::AutocompleteChoice<&'a str>> {
    ["en", "jp"]
        .iter()
        .map(|&s| poise::AutocompleteChoice {
            name: s.to_string(),
            value: s,
        })
}

/// Speak a string in a language (default: zh)
#[poise::command(slash_command, prefix_command)]
pub async fn speak(
    ctx: Context<'_>,
    #[description = "string"] str_to_say: String,
    #[description = "lang"]
    #[autocomplete = "language_autocomplete"]
    lang: Option<String>,
) -> Result<(), Error> {
    ctx.defer().await?;
    // vc_handle(ctx, autojoin: bool, f: impl FnOnce(Arc<Mutex<Call>>, Context<'_>) -> Fut) -> Fut
    vc_handle(ctx, true, |call, c| async move {
        let mut call = call.lock().await;
        let default_lang = "zh";
        let lang = match lang {
            Some(lang) => lang,
            None => default_lang.to_string(),
        };
        let tts_request = format!(
            "https://translate.google.com/translate_tts?ie=UTF-8&q={}&tl={}&client=tw-ob",
            str_to_say, lang
        );
        let source = match ytdl(tts_request).await {
            Ok(tts_source) => tts_source,
            Err(e) => {
                c.say(format!("Error: {}", e)).await?;
                return Ok(());
            }
        };
        call.play_source(source);
        ctx.say(format!("Speaking... {}", str_to_say)).await?;
        Ok(())
    })
    .await
}
