# tinytones

**tinytones** is a `#![no_std]` Rust crate for defining and playing musical tones and melodies in embedded systems. It provides:

- Enum-based musical pitches from C0 to B8
- Standard and dotted note durations
- Frequency access as `u32` or `f64`
- Iterator for (pitch, duration) tone playback

## Predefined tones in the crate

The crate includes a few predefined melodies for convenience:

```rust
use tinytones::{Tone, songs::happy_birthday};


let song = Tone::new(happy_birthday::TEMPO, happy_birthday::MELODY);
for (pitch, duration_ms) in song.iter() {
    // Send `pitch.freq_u32()` to a PWM
    // Wait for `duration_ms` using a delay
}
```


## Definiting your own Tone
You can define your own melody using the provided `Pitch` and `Duration` enums:

```rust
use tinytones::{
    note::{Duration::*, Pitch::*},
    Tone, Melody,
};

let melody = Melody(&[
    (C4, Quarter),
    (D4, Quarter),
    (E4, Half),
]);

let tempo = 120;

let tone = Tone::new(tempo, melody);

for (pitch, duration_ms) in tone.iter() {
    // Send `pitch.freq_u32()` to a PWM
    // Wait for `duration_ms` using a delay
}
```
