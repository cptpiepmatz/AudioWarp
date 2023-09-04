use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SampleRate(u32);

impl SampleRate {
    pub fn new(value: u32) -> Self {
        Self(value)
    }
}

impl From<SampleRate> for cpal::SampleRate {
    fn from(value: SampleRate) -> Self {
        cpal::SampleRate(value.0)
    }
}

impl From<cpal::SampleRate> for SampleRate {
    fn from(value: cpal::SampleRate) -> Self {
        SampleRate(value.0)
    }
}

impl Display for SampleRate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.0 % 1000 {
            0 => write!(f, "{:>2} kHz", self.0 / 1000),
            _ => write!(f, "{} Hz", self.0)
        }
    }
}

impl PartialEq<cpal::SampleRate> for SampleRate {
    fn eq(&self, other: &cpal::SampleRate) -> bool {
        self.0.eq(&other.0)
    }
}

impl PartialOrd<cpal::SampleRate> for SampleRate {
    fn partial_cmp(&self, other: &cpal::SampleRate) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
