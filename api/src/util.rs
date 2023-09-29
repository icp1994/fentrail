/// Create a PGN move string from a sequence of [`SanPlus`](shakmaty::san::SanPlus).
///
/// Caller is responsible for ensuring validity of the provided moves.
///
/// # Examples
///
/// ```
/// use libft::util::sp_seq_to_pgn;
/// use shakmaty::san::{SanPlus, ParseSanError};
///
/// fn sp_strs_to_pgn(sp_strs: Vec<&str>, pm: usize) -> Result<String, ParseSanError> {
///     sp_strs
///         .into_iter()
///         .map(|sp| sp.parse()).collect::<Result<Vec<SanPlus>, ParseSanError>>()
///         .and_then(|sp| Ok(sp_seq_to_pgn(&sp, pm)))
/// }
///
/// assert!(sp_seq_to_pgn(&[], 0).is_empty());
///
/// let sp = vec!["Nf3", "Nf6", "g3", "d5", "Bg2"];
/// assert_eq!(sp_strs_to_pgn(sp, 0)?, "1. Nf3 Nf6 2. g3 d5 3. Bg2");
///
/// let sp = vec!["Bb5", "a6", "Bxc6", "dxc6", "O-O"];
/// assert_eq!(sp_strs_to_pgn(sp, 4)?, "3. Bb5 a6 4. Bxc6 dxc6 5. O-O");
///
/// let sp = vec!["Bb4", "Qc2", "O-O", "a3", "Bxc3+"];
/// assert_eq!(sp_strs_to_pgn(sp, 5)?, "3... Bb4 4. Qc2 O-O 5. a3 Bxc3+");
///
/// # Ok::<(), ParseSanError>(())
/// ```
#[must_use]
pub fn sp_seq_to_pgn(moves: &[crate::SanPlus], plies_moved: usize) -> String {
    let mut out: Vec<String> = vec![];
    let mut count = (plies_moved + 1) / 2;

    if !moves.is_empty() && (plies_moved % 2 == 1) {
        out.push(format!("{count}...",));
    }

    for (idx, sp) in moves.iter().enumerate() {
        if (plies_moved + idx) % 2 == 0 {
            count += 1;
            out.push(format!("{count}."));
        }
        out.push(sp.to_string());
    }

    out.join(" ")
}
