#![no_std]

pub mod note;
pub mod prelude;
pub mod songs;
pub mod tones;

use crate::note::{Duration, Pitch};

/// A musical melody represented as a sequence of pitches and durations.
pub struct Melody(pub &'static [(Pitch, Duration)]);

/// A musical tone represents a melody with a given tempo (beats per minute).
pub struct Tone {
    whole_note: u64,
    melody: Melody,
}

impl Tone {
    /// Creates a new `Tone` with the specified tempo and melody.
    pub fn new(tempo: u16, melody: Melody) -> Self {
        let whole_note = (60_000 * 4) / tempo as u64;
        Self { whole_note, melody }
    }

    /// Convert a musical `Duration` to milliseconds based on the tone's tempo.
    pub fn duration_ms(&self, duration: Duration) -> u64 {
        match duration {
            Duration::Whole => self.whole_note,
            Duration::Half => self.whole_note / 2,
            Duration::Quarter => self.whole_note / 4,
            Duration::Eighth => self.whole_note / 8,
            Duration::Sixteenth => self.whole_note / 16,
            Duration::Dotted(n) => (self.whole_note / n as u64) * 3 / 2,
        }
    }

    /// Iterate over the melody, returning each note with its duration in milliseconds.
    pub fn iter(&self) -> impl Iterator<Item = (Pitch, u64)> {
        self.melody
            .0
            .iter()
            .map(move |(note, duration)| (*note, self.duration_ms(*duration)))
    }
}
