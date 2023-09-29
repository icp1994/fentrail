#![warn(clippy::pedantic)]

mod query;

const SERVEFT_DEFAULT_PORT: u16 = 8000;

fn main() -> anyhow::Result<()> {
    let port = std::env::var("SERVEFT_PORT").unwrap_or(SERVEFT_DEFAULT_PORT.to_string());
    astra::Server::bind(format!("localhost:{port}")).serve(|req, _info| {
        query::handle(req)
            .or_else(|_| query::ise())
            .expect("serve failed")
    })?;

    Ok(())
}
