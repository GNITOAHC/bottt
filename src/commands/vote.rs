use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn vote(
    ctx: Context<'_>,
    #[description = "title"] _title: Option<String>,
    #[description = "options"] _options: Option<String>,
) -> Result<(), Error> {
    ctx.say("Unimplemented").await?;
    Ok(())
}
