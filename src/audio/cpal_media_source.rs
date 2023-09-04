use std::io;
use std::io::{ErrorKind, Read, Seek, SeekFrom};
use std::sync::Arc;

use cpal::traits::DeviceTrait;
use cpal::{
    BuildStreamError, Device, FromSample, Sample, SizedSample, Stream, StreamConfig, StreamError
};
use crossbeam::atomic::AtomicCell;
use crossbeam::queue::ArrayQueue;
use songbird::input::core::io::MediaSource;
use songbird::input::{Input, RawAdapter};

/// Media Source that receives data via [cpal].
///
/// # Cloning
/// Every clone of this media source shares a reference to the [ArrayQueue] that
/// is producing the data.
/// Therefore would two instances of this try to pull data from the same queue.
/// Make sure you read only from one media source at a time to avoid audio
/// issues.
#[derive(Clone)]
pub struct CpalMediaSource {
    data: Arc<ArrayQueue<u8>>,
    error: Arc<AtomicCell<Option<StreamError>>>,
    sample_rate: u32,
    channel_count: u32
}

impl CpalMediaSource {
    const DATA_QUEUE_SIZE: usize = 8 * 1024 * 1024;

    pub fn from_device<T>(
        device: &Device,
        stream_config: &StreamConfig
    ) -> Result<(Self, Stream), BuildStreamError>
    where
        T: SizedSample,
        f32: FromSample<T>
    {
        let data_consumer = Arc::new(ArrayQueue::new(Self::DATA_QUEUE_SIZE));
        let data_producer = data_consumer.clone();
        let error = Arc::new(AtomicCell::new(None));
        let callback_error = error.clone();

        let input_stream = device.build_input_stream(
            stream_config,
            move |data: &[T], _| {
                for date in data.iter() {
                    // songbird needs data to be encoded directly as opus, raw i16 or raw f32
                    let sample: f32 = date.to_sample();
                    for byte in sample.to_ne_bytes() {
                        data_producer.force_push(byte);
                    }
                }
            },
            move |stream_error| callback_error.store(Some(stream_error)),
            None
        )?;

        Ok((
            CpalMediaSource {
                data: data_consumer,
                error,
                sample_rate: stream_config.sample_rate.0,
                channel_count: stream_config.channels as u32
            },
            input_stream
        ))
    }

    pub fn into_input(self) -> Input {
        let sample_rate = self.sample_rate;
        let channel_count = self.channel_count;
        RawAdapter::new(self, sample_rate, channel_count).into()
    }
}

impl Read for CpalMediaSource {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self.error.swap(None) {
            Some(StreamError::DeviceNotAvailable) => {
                return Err(io::Error::new(
                    ErrorKind::BrokenPipe,
                    "source got disconnected"
                ))
            }
            Some(err) => return Err(io::Error::new(ErrorKind::Other, err.to_string())),
            None => ()
        }

        // to keep performance as high as possible, this will just use the queue without
        // any checks other components need to make sure that not two reads
        // happen at the same time
        for (i, byte_ref) in buf.iter_mut().enumerate() {
            match (self.data.pop(), i) {
                (None, 0) => return Err(io::ErrorKind::WouldBlock.into()),
                (None, _) => return Ok(i),
                (Some(byte), _) => *byte_ref = byte
            }
        }

        Ok(buf.len())
    }
}

impl Seek for CpalMediaSource {
    fn seek(&mut self, _: SeekFrom) -> io::Result<u64> {
        // The source does not provide seekability but the trait
        // `MediaSource` requires this
        unimplemented!()
    }
}

impl MediaSource for CpalMediaSource {
    fn is_seekable(&self) -> bool {
        false
    }

    fn byte_len(&self) -> Option<u64> {
        Some(self.data.len() as u64)
    }
}
