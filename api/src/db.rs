use std::path::Path;

use redb::{Database, MultimapTableDefinition, ReadableMultimapTable};

use crate::error::Result;
use crate::model::{FenTrail, Trail};

const TABLE: MultimapTableDefinition<&[u8], &[u8]> = MultimapTableDefinition::new("fen-trail");

#[must_use]
#[derive(Debug)]
pub(crate) struct FenTrailStore {
    db: Database,
}

fn err_redb<E: Into<redb::Error>>(err: E) -> crate::Error {
    crate::error::Error::Redb(err.into())
}

impl FenTrailStore {
    pub fn conn<P: AsRef<Path>>(path: P) -> Result<Self> {
        let db = Database::create(&path).map_err(err_redb)?;
        Ok(FenTrailStore { db })
    }

    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let db = Database::open(&path).map_err(err_redb)?;
        Ok(FenTrailStore { db })
    }

    pub fn write<I: IntoIterator<Item = FenTrail> + Send>(&self, fentrails: I) -> Result<()> {
        let write_txn = self.db.begin_write().map_err(err_redb)?;

        {
            // New scope for `table` which borrows `write_txn` and implements `Drop`.
            let mut table = write_txn.open_multimap_table(TABLE).map_err(err_redb)?;
            for FenTrail { fen, trail } in fentrails {
                let trail_bytes = serde_json::to_vec(&trail)?;
                table
                    .insert(fen.to_string().as_bytes(), trail_bytes.as_slice())
                    .map_err(err_redb)?;
            }
        } // `write_txn` is now free to be consumed via committing.

        write_txn.commit().map_err(err_redb)
    }

    pub fn read(&self, fen: &crate::Fen) -> Result<Vec<Trail>> {
        let read_txn = self.db.begin_read().map_err(err_redb)?;
        let table = read_txn.open_multimap_table(TABLE).map_err(err_redb)?;

        let trails = table
            .get(fen.to_string().as_bytes())
            .map_err(err_redb)?
            .map(|trail| match trail {
                Ok(trail) => {
                    let trail = trail.value();
                    let errmsg = format!(
                        "Failed deserialization into `Trail` of value {trail:#?} for fen {fen:#?}"
                    );
                    let desererr = err_redb(redb::Error::Corrupted(errmsg));
                    serde_json::from_slice(trail).map_err(|_| desererr)
                }
                Err(e) => Err(err_redb(e)),
            });

        trails.collect()
    }
}
