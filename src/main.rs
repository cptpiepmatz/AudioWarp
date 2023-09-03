#![allow(non_snake_case)]

use std::borrow::Cow;
use std::env::VarError;
use std::io::Stdout;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{env, io, thread};
use std::collections::HashMap;
use std::future::IntoFuture;
use songbird::join::Join;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::SampleFormat;
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen
};
use crossterm::{event, execute};
use ratatui::prelude::CrosstermBackend;
use ratatui::widgets::Paragraph;
use ratatui::Terminal as RataTerminal;
use songbird::shards::TwilightMap;
use songbird::{Call, Songbird};
use songbird::driver::Bitrate;
use tokio::runtime::Runtime;
use twilight_gateway::{ConfigBuilder, Event as TwilightGatewayEvent, Shard};
use twilight_http::Client as HttpClient;
use twilight_model::gateway::{Intents, ShardId};
use twilight_model::gateway::payload::outgoing::update_presence::UpdatePresencePayload;
use twilight_model::gateway::presence::{Activity, ActivityEmoji, ActivityType, Status};

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
            let input = media_source.into_input();
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

        tokio::spawn(handle_event(event, ctx.clone()));
    }

    Ok(())
}

async fn handle_event(event: TwilightGatewayEvent, ctx: Arc<AppContext>) {
    use TwilightGatewayEvent as E;
    match event {
        E::GuildDelete(_) => {
            // TODO: disconnect from that voice, if connected
        }
        E::GuildUpdate(_) => {
            // TODO: update ui indicator
        }
        E::VoiceStateUpdate(_) => {
            // TODO: update ui indicator and songbird connection
        }
        E::InteractionCreate(_) => {
            // TODO: listen to commands
        }

        _ => ()
    }
}

// is public to allow 'ui::run_select' to be public
pub struct AppContext {
    pub http: HttpClient,
    pub songbird: Songbird
}

fn setup_terminal() -> anyhow::Result<Terminal> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore_terminal(terminal: &mut Terminal) -> anyhow::Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(terminal.show_cursor()?)
}

fn run(terminal: &mut Terminal) -> anyhow::Result<()> {
    loop {
        terminal.draw(|frame| {
            let greeting = Paragraph::new("Hello World");
            frame.render_widget(greeting, frame.size());
        })?;
        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if KeyCode::Char('q') == key.code {
                    break;
                }
            }
        }
    }

    Ok(())
}
