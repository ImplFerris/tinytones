use crate::{
    Melody,
    note::{Duration::*, Pitch::*},
};

pub const TEMPO: u16 = 140;

pub const MELODY: Melody = Melody(&[
    (C4, Quarter),
    (C4, Eighth),
    (D4, Dotted(4)),
    (C4, Dotted(4)),
    (F4, Dotted(4)),
    (E4, Dotted(2)),
    (C4, Quarter),
    (C4, Eighth),
    (D4, Dotted(4)),
    (C4, Dotted(4)),
    (G4, Dotted(4)),
    (F4, Dotted(2)),
    (C4, Quarter),
    (C4, Eighth),
    (C5, Dotted(4)),
    (A4, Dotted(4)),
    (F4, Dotted(4)),
    (E4, Dotted(4)),
    (D4, Dotted(4)),
    (AS4, Quarter),
    (AS4, Eighth),
    (A4, Dotted(4)),
    (F4, Dotted(4)),
    (G4, Dotted(4)),
    (F4, Dotted(2)),
]);
