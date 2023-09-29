use std::env;

use astra::{Body, Request, Response, ResponseBuilder};

const MAX_ALLOWED_REQUEST_SIZE: usize = 127;

pub fn handle(mut req: Request) -> anyhow::Result<Response> {
    let mut resp = ResponseBuilder::new();

    if req.method().as_str() == "POST" {
        let mut body = String::with_capacity(MAX_ALLOWED_REQUEST_SIZE);
        for chunk in req.body_mut() {
            body.push_str(std::str::from_utf8(chunk?.as_ref())?);
            if body.len() > MAX_ALLOWED_REQUEST_SIZE {
                break;
            }
        }

        if let Ok(fen) = body.parse() {
            let store_path: std::path::PathBuf;
            if let Ok(val) = env::var("SERVEFT_STORE") {
                store_path = val.into();
            } else {
                store_path = env::current_dir()?.join("fentrail.redb");
            }

            assert!(store_path.exists());

            let asker = libft::Asker { fen, store_path };
            let trails = serde_json::to_string(&asker.ask()?)?;

            resp = resp.header("Content-Type", "application/json");
            Ok(resp.body(Body::new(trails))?)
        } else {
            resp = resp.status(400);
            Ok(resp.body(Body::empty())?)
        }
    } else {
        resp = resp.status(405).header("Allow", "POST");
        Ok(resp.body(Body::empty())?)
    }
}

pub fn ise() -> anyhow::Result<Response> {
    Ok(ResponseBuilder::new().status(500).body(Body::empty())?)
}
