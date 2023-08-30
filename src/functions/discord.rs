use log::error;
use serenity::{async_trait, http::Http, model::id::ChannelId, prelude::*};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        let _ = ctx.set_activity(Activity::playing("Load Balancer"));
    }
}

pub async fn discord_notify(channel_id: ChannelId, message: String) {
    let http = Http::new_with_token("YOUR_DISCORD_BOT_TOKEN");
    if let Err(why) = channel_id.send_message(&http, |m| m.content(message)).await {
        error!("Error sending message: {:?}", why);
    }
}
