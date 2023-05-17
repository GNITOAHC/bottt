use crate::{Context, Error};
use poise::serenity_prelude::ReactionType;
use serenity::http::CacheHttp;

#[poise::command(slash_command)]
pub async fn vote(
    ctx: Context<'_>,
    #[description = "title"] title: String,
    #[description = "options"] options: String,
) -> Result<(), Error> {
    let options_arr: Vec<&str> = options.split(",").collect();
    let iter_head: Vec<(&str, &str)> = vec![
        (":one:", "1️⃣"),
        (":two:", "2️⃣"),
        (":three:", "3️⃣"),
        (":four:", "4️⃣"),
        (":five:", "5️⃣"),
        (":six:", "6️⃣"),
        (":seven:", "7️⃣"),
        (":eight:", "8️⃣"),
        (":nine:", "9️⃣"),
        (":keycap_ten:", "🔟"),
    ];

    let reply = ctx
        .send(|f| {
            f.embed(|e| {
                e.title("🙋‍♂️Vote".to_string());
                e.description(title.to_string().bold());
                for (i, option) in options_arr.iter().enumerate() {
                    e.field("", format!("{} {}", iter_head[i].0, option), false);
                }
                e.timestamp(chrono::Utc::now().to_rfc3339());
                e
            })
        })
        .await?;

    let reply_id = match reply.into_message().await {
        Ok(msg) => msg.id,
        Err(_) => panic!("Failed to get message id"),
    };

    for i in 0..options_arr.len() {
        let react_unicode = ReactionType::Unicode(iter_head[i].1.to_string());
        ctx.http()
            .create_reaction(ctx.channel_id().into(), reply_id.into(), &react_unicode)
            .await?;
    }

    Ok(())
}

pub trait Bold {
    fn bold(&self) -> String;
}

impl Bold for String {
    fn bold(&self) -> String {
        format!("**{}**", self)
    }
}
