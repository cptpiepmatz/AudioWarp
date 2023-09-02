use std::time::Duration;
use std::{cmp, process};

use cpal::{Device, StreamConfig};
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState};

use crate::audio::InputDeviceListItem;
use crate::{AppContext, Terminal};

/// UI-loop for the select menu.
pub fn run_select(
    terminal: &mut Terminal,
    ctx: &AppContext,
    input_devices: &[InputDeviceListItem]
) -> anyhow::Result<(Device, StreamConfig)> {
    let mut list_input_device_state = ListState::default().with_selected(
        input_devices
            .iter()
            .enumerate()
            .find(|(_, item)| item.is_default)
            .map(|(index, _)| index)
    );

    loop {
        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(frame.size());

            let (left, right) = (chunks[0], chunks[1]);
            let input_device_list = List::new(
                input_devices
                    .iter()
                    .map(|item| &item.name)
                    .map(|name| ListItem::new(name.to_owned()))
                    .collect::<Vec<_>>()
            )
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("select audio input device")
            )
            .highlight_symbol("> ");
            frame.render_stateful_widget(input_device_list, left, &mut list_input_device_state);
        })?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(KeyEvent {
                code,
                kind: KeyEventKind::Press,
                ..
            }) = event::read()?
            {
                match code {
                    // TODO: make this a cleaner exit
                    KeyCode::Char('q') => process::exit(0),

                    KeyCode::Up => {
                        let max_index = input_devices.len() - 1;
                        let new_index = match list_input_device_state.selected() {
                            None => unreachable!("state init with select"),
                            Some(0) => max_index,
                            Some(u) => u - 1
                        };
                        list_input_device_state.select(Some(new_index));
                    }
                    KeyCode::Down => {
                        let max_index = input_devices.len() - 1;
                        let new_index = match list_input_device_state.selected() {
                            None => unreachable!("state init with select"),
                            Some(u) if u == max_index => 0,
                            Some(u) => u + 1
                        };
                        list_input_device_state.select(Some(new_index));
                    }

                    KeyCode::Left => todo!(),
                    KeyCode::Right => todo!(),

                    _ => ()
                }
            }
        }
    }

    todo!()
}

/// UI-loop for the main screen.
pub fn run_main(terminal: &mut Terminal) -> anyhow::Result<()> {
    todo!()
}
