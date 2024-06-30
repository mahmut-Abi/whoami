use serde::{Deserialize, Serialize};
use worker::*;

#[derive(Debug, Deserialize, Serialize)]
struct GenericResponse {
    status: u16,
    message: String,
}

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();
    router
        .get_async("/", root_handle_request)
        .run(req, env)
        .await
}

async fn root_handle_request(req: Request, _ctx: RouteContext<()>) -> worker::Result<Response> {
    console_log!("{:?}", req);

    let req_method = req.method();
    console_debug!("request method: {}", req_method);

    let headers = req.headers();

    let host: String = match headers.get("host") {
        Ok(host) => match host {
            Some(a) => a,
            None => "".to_string(),
        },
        Err(_e) => "".to_string(),
    };

    console_debug!("host: {}", host);

    Response::from_json(&GenericResponse {
        status: 200,
        message: "Hi!".to_string(),
    })
}
