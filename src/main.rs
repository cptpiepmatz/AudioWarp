#![allow(non_snake_case)]

use std::borrow::Cow;
use std::env::VarError;
use std::io::Stdout;
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
use twilight_gateway::Shard;
use twilight_http::Client as HttpClient;
use twilight_model::gateway::{Intents, ShardId};

use crate::audio::CpalMediaSource;

type Terminal = RataTerminal<CrosstermBackend<Stdout>>;

mod audio;
mod discord;
mod select;
mod ui;

// TODO: make this infallible
fn main() -> anyhow::Result<()> {
    let token: Cow<str> = match (env::var("TOKEN"), env!("INCLUDED_TOKEN")) {
        (Ok(token), "") if token.as_str() == "" => panic!("empty token"),
        (Ok(token), _) => token.into(),
        (Err(VarError::NotPresent), "") => panic!("no token"),
        (Err(VarError::NotPresent), token) => token.into(),
        (Err(VarError::NotUnicode(_)), _) => panic!("the token in `TOKEN` is not utf-8")
    };

    let ctx = AppContext {};

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

    Ok(())

    // let mut terminal = setup_terminal()?;
    // let input_devices = audio::list_input_devices()?;
    // ui::run_select(&mut terminal, &ctx, input_devices.as_slice())?;
    // restore_terminal(&mut terminal)?;
    // Ok(())

    // let mut shard = Shard::new(
    //     ShardId::ONE,
    //     token.to_string(),
    //     Intents::GUILD_VOICE_STATES
    // );
    //
    // let http = Arc::new(HttpClient::new(token.to_string()));
}

// is public to allow 'ui::run_select' to be public
pub struct AppContext {}

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
