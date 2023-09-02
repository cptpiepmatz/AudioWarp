use std::borrow::Cow;
use std::collections::HashSet;
use std::iter;
use std::ops::Deref;

use console::Term;
use cpal::traits::DeviceTrait;
use cpal::{BufferSize, ChannelCount, Device, StreamConfig};
use dialoguer::theme::{ColorfulTheme, Theme};
use dialoguer::Select;
use lazy_static::lazy_static;

use crate::audio::{InputDeviceListItem, SampleRate};

lazy_static! {
    static ref THEME: ColorfulTheme = ColorfulTheme::default();
}

pub fn select_input_device(
    from_devices: Vec<InputDeviceListItem>
) -> anyhow::Result<(Device, String)> {
    let theme = THEME.deref();

    let mut select = Select::with_theme(theme);
    select.with_prompt("Select Audio Device");
    select.items(from_devices.as_slice());
    if let Some(default_index) = from_devices
        .iter()
        .enumerate()
        .find(|(_, item)| item.is_default)
        .map(|(index, _)| index)
    {
        select.default(default_index);
    }

    let index = select.interact()?;
    let InputDeviceListItem { device, name, .. } = from_devices
        .into_iter()
        .nth(index)
        .expect("index evaluated from this list");
    Ok((device, name))
}

pub fn select_stream_config(selected_input_device: &Device) -> anyhow::Result<StreamConfig> {
    let theme = THEME.deref();

    let input_configs: Vec<_> = selected_input_device.supported_input_configs()?.filter(|range| range.channels() <= 2).collect();
    let selected_config = match input_configs.len() {
        0 => return Err(anyhow::Error::msg("no supported input config found that is compatible with AudioWarp")),
        1 => input_configs.into_iter().next().expect("not empty"),
        _ => {
            let input_config_channels: Vec<_> = input_configs.iter().map(|range| match range.channels() {
                0 => unimplemented!(),
                1 => Cow::Borrowed("mono"),
                2 => Cow::Borrowed("stereo"),
                c => Cow::Owned(c.to_string())
            }).collect();
            let mut select = Select::with_theme(theme);
            select.with_prompt("Select Channel Count").items(input_config_channels.as_slice());
            input_configs.iter().enumerate().find(|(_, range)| range.channels() == 2).map(|(i, _)| i).iter().for_each(|i| { select.default(*i);});
            let index = select.interact()?;

            input_configs.into_iter().nth(index).expect("index is from this vec")
        }
    };

    let max_sample_rate = selected_config.max_sample_rate();
    let min_sample_rate = selected_config.min_sample_rate();
    let sample_rate = match max_sample_rate == min_sample_rate {
        true => max_sample_rate,
        false => {
            let rates: Vec<_> = [("max", max_sample_rate), ("min", min_sample_rate)].iter().map(|(name, rate)| format!("{} {}", name, SampleRate::from(*rate))).collect();
            let index = Select::with_theme(theme).with_prompt("Select Sample Rate").items(rates.as_slice()).default(0).interact()?;
            match index {
                0 => max_sample_rate,
                1 => min_sample_rate,
                _ => unreachable!()
            }
        }
    };

    Ok(StreamConfig {
        channels: selected_config.channels(),
        sample_rate,
        buffer_size: BufferSize::Default
    })
}
