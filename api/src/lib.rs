#![warn(clippy::pedantic)]

mod db;
mod eco;
mod error;

mod asker;
mod packer;

pub mod model;
pub mod util;

pub use error::Error;

pub use asker::Asker;
pub use packer::Packer;

#[doc(no_inline)]
pub use shakmaty::fen::Fen;
#[doc(no_inline)]
pub use shakmaty::san::SanPlus;
