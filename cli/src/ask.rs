use crate::model::AskCmd;

pub fn run(ask_cmd: AskCmd) -> anyhow::Result<()> {
    let AskCmd { store, fen } = ask_cmd;

    let asker = libft::Asker {
        fen: fen.parse()?,
        store_path: store.unwrap_or(std::env::current_dir()?.join("fentrail.redb")),
    };

    let trails = asker.ask()?;
    for trail in trails {
        println!("{}", serde_json::to_string(&trail)?);
    }

    Ok(())
}
