use crate::{
    Melody,
    note::{Duration::*, Pitch::*},
};

pub const TEMPO: u16 = 120;

pub const MELODY: Melody = Melody(&[
    (E4, Quarter),
    (E4, Quarter),
    (E4, Half),
    (E4, Quarter),
    (E4, Quarter),
    (E4, Half),
    (E4, Quarter),
    (G4, Quarter),
    (C4, Quarter),
    (D4, Quarter),
    (E4, Whole),
    (F4, Quarter),
    (F4, Quarter),
    (F4, Quarter),
    (F4, Quarter),
    (F4, Quarter),
    (E4, Quarter),
    (E4, Quarter),
    (E4, Eighth),
    (E4, Eighth),
    (E4, Quarter),
    (D4, Quarter),
    (D4, Quarter),
    (E4, Quarter),
    (D4, Half),
    (G4, Half),
]);
