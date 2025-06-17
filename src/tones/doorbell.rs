use crate::{
    Melody,
    note::{Duration::*, Pitch::*},
};

pub const TEMPO: u16 = 100;

pub const MELODY: Melody = Melody(&[
    (C5, Eighth),
    (E5, Eighth),
    (G5, Eighth),
    (C6, Eighth),
    (G5, Eighth),
    (E5, Eighth),
    (C5, Eighth),
]);
