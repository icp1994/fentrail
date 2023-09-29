/// Superset of all errors that can occur.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Tsv(#[from] csv::Error),
    #[error(transparent)]
    Redb(#[from] redb::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    ParseFen(#[from] shakmaty::fen::ParseFenError),
    #[error(transparent)]
    ParseSan(#[from] shakmaty::san::ParseSanError),
}

pub type Result<T> = std::result::Result<T, Error>;
