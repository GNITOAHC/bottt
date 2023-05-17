use crate::{Context, Error};
use poise::serenity_prelude::AttachmentType;
use tokio::fs::File;

async fn lovebear_autocomplete<'a>(
    _ctx: Context<'_>,
    _partial: &'a str,
) -> impl Iterator<Item = poise::AutocompleteChoice<&'a str>> {
    [
        ("今天很乖", "good.jpg"),
        ("冷落我", "you_bad_bad.jpg"),
        ("扣錯了", "sorry.jpg"),
        ("不給瑟瑟", "no_hs_hs.jpg"),
        ("欺負我", "you_bully_me.jpg"),
        ("跟其他女生講話", "talking_to_other_girl.jpg"),
        ("不回訊息", "you_dont_answer_me.jpg"),
    ]
    .iter()
    .map(|&n| poise::AutocompleteChoice {
        name: format!("{}", n.0),
        value: n.1,
    })
}

async fn xi_autocomplete<'a>(
    _ctx: Context<'_>,
    _partial: &'a str,
) -> impl Iterator<Item = poise::AutocompleteChoice<&'a str>> {
    [("nono", "jinping_dont_like_this.jpg")]
        .iter()
        .map(|&n| poise::AutocompleteChoice {
            name: format!("{}", n.0),
            value: n.1,
        })
}

#[poise::command(slash_command)]
pub async fn what(
    ctx: Context<'_>,
    #[description = "lovebear"]
    #[autocomplete = "lovebear_autocomplete"]
    lovebear: Option<String>,
    #[description = "xi"]
    #[autocomplete = "xi_autocomplete"]
    xi: Option<String>,
) -> Result<(), Error> {
    let mut file = ctx.data().assets.clone();
    file.push("images");
    if let Some(lovebear) = &lovebear {
        file.push("lovebear");
        file.push(lovebear.as_str());
        let path = File::open(file.as_path()).await?;
        let return_file = AttachmentType::File {
            file: &path,
            filename: lovebear.to_string(),
        };
        ctx.send(|f| {
            f.attachment(return_file);
            f
        })
        .await?;
    }

    if let Some(xi) = &xi {
        file.push("xi");
        file.push(xi.as_str());
        let path = File::open(file.as_path()).await?;
        let return_file = AttachmentType::File {
            file: &path,
            filename: xi.to_string(),
        };
        ctx.send(|f| {
            f.attachment(return_file);
            f
        })
        .await?;
    }

    Ok(())
}
