use worker::*;

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();
    router
        .get_async("/", root_handle_request)
        .run(req, env)
        .await
}

async fn root_handle_request(req: Request, _ctx: RouteContext<()>) -> worker::Result<Response> {
    console_debug!("headers: {:?}", req.headers());

    let client_ip = req
        .headers()
        .get("CF-Connecting-IP")
        .or(req.headers().get("X-Forwarded-For"))
        .or(req.headers().get("True-Client-IP"))
        .or(req.headers().get("X-Real-IP"))
        .or(req.headers().get("Forwarded"))
        .ok()
        .flatten()
        .unwrap_or_else(|| "Unknown IP".to_string());

    console_log!("Client IP: {}", client_ip);

    Response::ok(client_ip)
}
