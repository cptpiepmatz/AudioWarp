#![allow(non_snake_case)]

use std::borrow::Cow;
use std::env::VarError;
use std::io::Stdout;
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;
use std::{env, io};

use cpal::traits::{DeviceTrait, HostTrait};
use cpal::SampleFormat;
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen
};
use crossterm::{event, execute};
use ratatui::prelude::CrosstermBackend;
use ratatui::widgets::Paragraph;
use ratatui::Terminal as RataTerminal;
use twilight_gateway::{Event as TwilightGatewayEvent, Shard};
use twilight_http::Client as HttpClient;
use twilight_model::gateway::{Intents, ShardId};

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

    for config in input_device.supported_input_configs()? {
        dbg!(config);
    }

    let stream_config = select::select_stream_config(&input_device)?;
    let (media_source, stream) =
        // TODO: this is static, maybe in the future this needs to be dynamic
        CpalMediaSource::from_device::<f32>(&input_device, &stream_config)?;

    let mut shard = Shard::new(ShardId::ONE, token.to_string(), Intents::GUILD_VOICE_STATES);

    let http = HttpClient::new(token.to_string());

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

    let ctx = Arc::new(AppContext { http });

    let selected_to_join = match init_guilds {
        None => None,
        Some(init_guilds) => select::select_channel_to_join(init_guilds, &ctx).await?
    };

    if let Some((guild, channel)) = selected_to_join {
        dbg!(&guild.name);
        dbg!(&channel.name);
    }

    std::process::exit(0);

    loop {
        let event = match shard.next_event().await {
            Ok(event) => event,
            Err(source) => {
                if source.is_fatal() {
                    break;
                }

                continue;
            }
        };

        tokio::spawn(handle_event(event, ctx.clone()));
    }

    Ok(())
}

async fn handle_event(event: TwilightGatewayEvent, ctx: Arc<AppContext>) {
    use TwilightGatewayEvent as E;
    match event {
        E::Ready(event) => {
            let event = event.deref();
            println!("ready.guilds: {}", event.guilds.len());
        }
        _ => ()
    }

    // dbg!(event);
}

// is public to allow 'ui::run_select' to be public
pub struct AppContext {
    pub http: HttpClient
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
