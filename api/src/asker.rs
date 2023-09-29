/// Query for the stored database.
#[must_use]
#[derive(Debug)]
pub struct Asker {
    /// Position to query, represented as a
    /// [FEN](https://www.chessprogramming.org/Forsyth-Edwards_Notation)
    pub fen: crate::Fen,
    /// Path to the stored database
    pub store_path: std::path::PathBuf,
}

impl Asker {
    /// Retrieve the set of [Trail](`crate::model::Trail`) from the
    /// [store](`Asker::store_path`) which corresponds to the provided
    /// [FEN](`Asker::fen`).
    ///
    /// # Errors
    ///
    /// Runtime errors are forwarded to [`crate::Error`].
    pub fn ask(&self) -> crate::error::Result<Vec<crate::model::Trail>> {
        let db = crate::db::FenTrailStore::open(&self.store_path)?;
        db.read(&self.fen)
    }
}
