#![allow(non_snake_case)]

use std::borrow::Cow;
use std::env::VarError;
use std::io::Stdout;
use std::ops::Deref;
use std::sync::Arc;
use std::env;
use std::collections::HashMap;
use std::future::IntoFuture;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use ratatui::prelude::CrosstermBackend;
use ratatui::Terminal as RataTerminal;
use songbird::shards::TwilightMap;
use songbird::Songbird;
use songbird::driver::Bitrate;
use twilight_gateway::{ConfigBuilder, Shard};
use twilight_http::Client as HttpClient;
use twilight_model::gateway::{Intents, ShardId};
use twilight_model::gateway::payload::outgoing::update_presence::UpdatePresencePayload;
use twilight_model::gateway::presence::{Activity, ActivityType, Status};

use crate::audio::CpalMediaSource;

type Terminal = RataTerminal<CrosstermBackend<Stdout>>;

mod audio;
mod discord;
mod select;
mod ui;

/// The maximum amount for when the client should request guild names.
const INIT_GUILD_REQ_THRESHOLD: usize = 10;

// TODO: make this infallible
#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let token: Cow<str> = match (env::var("TOKEN"), env!("INCLUDED_TOKEN")) {
        (Ok(token), "") if token.as_str() == "" => panic!("empty token"),
        (Ok(token), _) => token.into(),
        (Err(VarError::NotPresent), "") => panic!("no token"),
        (Err(VarError::NotPresent), token) => token.into(),
        (Err(VarError::NotUnicode(_)), _) => panic!("the token in `TOKEN` is not utf-8")
    };

    let input_devices = audio::list_input_devices()?;
    if input_devices.is_empty() {
        panic!("no input audio device found!");
    }
    let (input_device, input_device_name) = select::select_input_device(input_devices)?;

    let stream_config = select::select_stream_config(&input_device)?;
    let (media_source, stream) =
        // TODO: this is static, maybe in the future this needs to be dynamic
        CpalMediaSource::from_device::<f32>(&input_device, &stream_config)?;

    let activity: Activity = Activity {
        application_id: None,
        assets: None,
        buttons: Vec::new(),
        created_at: None,
        details: None,
        emoji: None,
        flags: None,
        id: None,
        instance: None,
        kind: ActivityType::Custom,
        name: String::from("AudioWarp"),
        party: None,
        secrets: None,
        state: String::from("✨ Warping 𝐀𝐮𝐝𝐢𝐨").into(),
        timestamps: None,
        url: None,
    };

    let mut shard = Shard::with_config(
        ShardId::ONE,
        ConfigBuilder::new(token.to_string(), Intents::GUILD_VOICE_STATES).presence(UpdatePresencePayload {
            activities: vec![activity],
            afk: false,
            since: None,
            status: Status::Online,
        }).build()
    );


    let http = HttpClient::new(token.to_string());
    let user_id = http.current_user().await?.model().await?.id;

    let ready = discord::wait_for_ready(&mut shard).await?;
    let guild_count = ready.guilds.len();

    let init_guilds = match guild_count {
        0 => None,
        l if l > INIT_GUILD_REQ_THRESHOLD => None,
        _ => {
            let mut guilds = Vec::with_capacity(guild_count);
            for guild in ready.guilds {
                let guild = http.guild(guild.id).await?.model().await?;
                guilds.push(guild);
            }
            Some(guilds)
        }
    };

    let twilight_map = TwilightMap::new(HashMap::from([(shard.id().number(), shard.sender())]));
    let songbird = Songbird::twilight(Arc::new(twilight_map), user_id);

    let ctx = Arc::new(AppContext { http, songbird });

    let selected_to_join = match init_guilds {
        None => None,
        Some(init_guilds) => select::select_channel_to_join(init_guilds, &ctx).await?
    };

    if let Some((guild, channel)) = selected_to_join {
        let ctx = ctx.clone();
        tokio::spawn(async move {
            // usually we check if we have only one call, but here it has to be empty
            let call = ctx.songbird.join(guild.id, channel.id).await.unwrap();
            let mut call = call.lock().await;
            call.deafen(true).await.unwrap();
            call.set_bitrate(Bitrate::Max);
            let input = media_source.clone().into_input();
            call.play_only_input(input);
        });
    }

    stream.play()?;

    loop {
        let event = match shard.next_event().await {
            Ok(event) => event,
            Err(source) if source.is_fatal() => break,
            Err(_) => continue,
        };

        ctx.songbird.process(&event).await;

        tokio::spawn(discord::handle_event(event, ctx.clone()));
    }

    Ok(())
}

// is public to allow 'ui::run_select' to be public
pub struct AppContext {
    pub http: HttpClient,
    // TODO: make a wrapper around Songbird to prevent having multiple Call instances
    pub songbird: Songbird
}
