use crate::model::PackCmd;

const PACKER_DEFAULT_DEPTH: u8 = 24;

pub fn run(pack_cmd: PackCmd) -> anyhow::Result<()> {
    let PackCmd {
        depth,
        ecotsv,
        outdir,
        pgn,
    } = pack_cmd;

    let packer = libft::Packer {
        depth: depth.map_or(PACKER_DEFAULT_DEPTH, |nb_moves| 2 * nb_moves),
        pgn_path: pgn,
        eco_path: ecotsv,
        store_path: outdir
            .unwrap_or(std::env::current_dir()?)
            .join("fentrail.redb"),
    };

    packer.pack()?;

    Ok(())
}
