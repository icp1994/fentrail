use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use shakmaty::Chess;

use crate::{Fen, SanPlus};

/// Chess opening characterized with a name and a sequence of
/// [SAN](https://www.chessprogramming.org/Algebraic_Chess_Notation).
#[must_use]
#[serde_as]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Opening {
    pub name: String,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    pub moves: Vec<SanPlus>,
}

/// Abstraction for a standard chess position.
///
/// The sequence of moves starting from the
/// [initial setup](https://www.chessprogramming.org/Initial_Position)
/// leading to a position is divided into two parts -
/// an [Opening](crate::model::Opening) and a possibly empty continuation.
#[must_use]
#[serde_as]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Trail {
    pub opening: Opening,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    pub continuation: Vec<SanPlus>,
}

#[must_use]
#[derive(Debug, Default)]
pub(crate) struct FenTrail {
    pub fen: Fen,
    pub trail: Trail,
}

#[must_use]
#[derive(Debug, Default)]
pub(crate) struct Eco {
    pub map: std::collections::HashMap<Fen, Opening>,
}

#[must_use]
#[derive(Debug, Default)]
pub(crate) struct TrailBlazer {
    pub ply: u8,
    pub depth: u8,
    pub valid: bool,
    pub eco: Eco,
    pub pos: Chess,
    pub trail: Trail,
    pub fentrails: Vec<FenTrail>,
}
