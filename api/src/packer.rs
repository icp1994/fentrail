use std::mem;

use std::fs::File;
use std::path::PathBuf;

use rayon::prelude::*;

use pgn_reader::{BufferedReader, RawHeader, Skip, Visitor};
use shakmaty::{Chess, EnPassantMode, Position};

use crate::model::{Eco, FenTrail, Trail, TrailBlazer};
use crate::{Fen, SanPlus};

/// Configuration to initiate/stack the database.
#[must_use]
#[derive(Debug)]
pub struct Packer {
    /// Number of plies to process in each game
    pub depth: u8,
    /// Path to the game database to process
    pub pgn_path: PathBuf,
    /// Path to the output database
    pub store_path: PathBuf,
    /// Path to a custom [ECO](https://www.chessprogramming.org/ECO) to use, if any
    pub eco_path: Option<PathBuf>,
}

impl Visitor for TrailBlazer {
    type Result = Vec<FenTrail>;

    fn begin_game(&mut self) {
        self.ply = 0;
        self.valid = true;
        self.pos = Chess::default();
        self.trail = Trail::default();
    }

    fn header(&mut self, key: &[u8], value: RawHeader<'_>) {
        // Invalidate non-standard variations and starting positions.
        if (key == b"FEN") || (key == b"Variation" && value.as_bytes() != b"Standard") {
            self.valid = false;
        }
    }

    fn san(&mut self, san_plus: SanPlus) {
        if (self.depth > self.ply) && self.valid {
            self.ply += 1;

            if let Ok(m) = san_plus.san.to_move(&self.pos) {
                self.pos.play_unchecked(&m);

                let fen = Fen::from_position(self.pos.clone(), EnPassantMode::Legal);
                if let Some(opening) = self.eco.map.get(&fen) {
                    // If a position itself is defined as a specific opening,
                    // the trail has no continuation.
                    self.trail.opening = opening.clone();
                    self.trail.continuation.clear();
                } else {
                    // Every position must arise from some opening.
                    debug_assert!(!self.trail.opening.name.is_empty());
                    self.trail.continuation.push(san_plus);
                }

                self.fentrails.push(FenTrail {
                    fen,
                    trail: self.trail.clone(),
                });
            } else {
                self.valid = false;
            }
        }
    }

    fn begin_variation(&mut self) -> Skip {
        Skip(true)
    }

    fn end_game(&mut self) -> Self::Result {
        mem::take(&mut self.fentrails)
    }
}

impl Packer {
    /// Populate [store](`Packer::store_path`) with games from a
    /// [pgn](`Packer::pgn_path`). Each game is processed till the provided
    /// [depth](`Packer::depth`). If a custom [ECO](`Packer::eco_path`) is
    /// supplied via a `tsv` file, it is used instead of the pre-defined openings.
    ///
    /// # Errors
    ///
    /// Runtime errors are forwarded to [`crate::Error`].
    pub fn pack(self) -> crate::error::Result<()> {
        let eco = self.eco_path.map_or_else(Eco::use_bundled, |p| {
            Eco::try_from_tsvreader(File::open(p)?)
        })?;

        let mut tb = TrailBlazer {
            eco,
            depth: self.depth,
            ..Default::default()
        };

        let db = crate::db::FenTrailStore::conn(&self.store_path)?;
        let (sender, receiver) = crossbeam_channel::unbounded();
        let handle = std::thread::spawn(move || db.write(receiver));

        let pgnfile = File::open(&self.pgn_path)?;
        let reader = BufferedReader::new(&pgnfile);

        reader
            .into_iter(&mut tb)
            .par_bridge()
            .flatten() // <Item = Result<Vec<FenTrail>, _>> -> <Item = Vec<FenTrail>>
            .flatten() // <Item = Vec<FenTrail>> -> <Item = FenTrail>
            .for_each(|fentrail| {
                if let Err(e) = sender.send(fentrail) {
                    eprintln!("{e}");
                }
            });

        drop(sender); // disconnect the channel to stop iteration
        if handle.join().is_err() {
            eprintln!("Could not join the database writer thread");
        }

        Ok(())
    }
}
