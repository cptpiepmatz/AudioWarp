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

    let max_supported_channels = selected_input_device
        .supported_input_configs()?
        .map(|range| range.channels())
        .max()
        .ok_or(anyhow::Error::msg("no channels available"))?;

    let is_stereo = match max_supported_channels {
        0 => unimplemented!("no channels are unreasonable"),
        1 => false,
        _ => {
            Select::with_theme(theme)
                .with_prompt("Select Audio Channels")
                .items(&["stereo", "mono"])
                .default(0)
                .interact()? ==
                0
        }
    };
    let channel_count = match is_stereo {
        true => 2,
        false => 1
    };

    dbg!(is_stereo);

    let min_sample_rate = selected_input_device
        .supported_input_configs()?
        .filter(|range| range.channels() >= channel_count)
        .map(|range| range.min_sample_rate())
        .min()
        .ok_or(anyhow::Error::msg(
            "no available supported config remaining"
        ))?;
    let max_sample_rate = selected_input_device
        .supported_input_configs()?
        .filter(|range| range.channels() >= channel_count)
        .map(|range| range.max_sample_rate())
        .max()
        .expect("if a minimum config is available, a max is as well");

    let valid_opus_sample_rates: Vec<_> = [
        SampleRate::new(48_000),
        SampleRate::new(24_000),
        SampleRate::new(16_000),
        SampleRate::new(12_000),
        SampleRate::new(8_000)
    ]
    .into_iter()
    .filter(|rate| rate <= &min_sample_rate && rate >= &max_sample_rate)
    .collect();

    let sample_rate = match valid_opus_sample_rates.len() {
        0 => return Err(anyhow::Error::msg("no fitting sample rate found")),
        1 => valid_opus_sample_rates[0],
        _ => {
            valid_opus_sample_rates[Select::with_theme(theme)
                .with_prompt("Select Sample Rate")
                .items(&valid_opus_sample_rates)
                .default(0)
                .interact()?]
        }
    };

    Ok(StreamConfig {
        channels: match is_stereo {
            true => 2,
            false => 1
        },
        sample_rate: sample_rate.into(),
        buffer_size: BufferSize::Default
    })
}
