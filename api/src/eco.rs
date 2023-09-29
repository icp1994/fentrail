use pgn_reader::{Skip, Visitor};
use shakmaty::{Chess, EnPassantMode, Position};

use crate::error::Result;
use crate::model::{Eco, Opening};
use crate::{Fen, SanPlus};

#[must_use]
#[derive(Debug, serde::Deserialize)]
struct TsvRow {
    name: String,
    pgn: String,
}

impl Visitor for Opening {
    type Result = Option<Fen>;

    fn san(&mut self, san_plus: SanPlus) {
        self.moves.push(san_plus);
    }

    fn begin_variation(&mut self) -> Skip {
        Skip(true)
    }

    fn end_game(&mut self) -> Self::Result {
        let mut pos = Chess::default();
        for sp in &self.moves {
            if let Ok(m) = sp.san.to_move(&pos) {
                pos.play_unchecked(&m);
            } else {
                return None;
            }
        }

        Some(Fen::from_position(pos, EnPassantMode::Legal))
    }
}

impl Eco {
    fn is_valid(&self) -> bool {
        // All possible first moves must be covered.
        for first_move in Chess::default().legal_moves() {
            let mut pos = Chess::default();
            pos.play_unchecked(&first_move);

            let fen = Fen::from_position(pos, EnPassantMode::Legal);
            if !self.map.contains_key(&fen) {
                return false;
            }
        }

        // All openings must have a name.
        for opening in self.map.values() {
            if opening.name.trim().is_empty() {
                return false;
            }
        }

        true
    }

    pub fn try_from_tsvreader<R: std::io::Read>(reader: R) -> Result<Eco> {
        let mut tsv_reader = csv::ReaderBuilder::new()
            .delimiter(b'\t')
            .from_reader(reader);

        let mut map = std::collections::HashMap::<Fen, Opening>::default();
        for record in tsv_reader.deserialize() {
            let tsv_row: TsvRow = record?;
            let mut opening = Opening {
                name: tsv_row.name,
                moves: vec![],
            };

            let mut pgn_reader = pgn_reader::BufferedReader::new_cursor(&tsv_row.pgn);
            if let Some(optfen) = pgn_reader.read_game(&mut opening)? {
                optfen.and_then(|fen| map.insert(fen, opening));
            }
        }

        let eco = Eco { map };
        assert!(eco.is_valid());

        Ok(eco)
    }

    pub fn use_bundled() -> Result<Eco> {
        Self::try_from_tsvreader(&include_bytes!("../eco.tsv")[..])
    }
}
