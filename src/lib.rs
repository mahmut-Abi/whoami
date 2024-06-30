use worker::*;

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();
    router
        .get_async("/", root_handle_request)
        .run(req, env)
        .await
}

async fn root_handle_request(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    console_log!("{:?}", req);

    let req_method = req.method();
    console_debug!("request method: {}", req_method);

    // Extract the accept-language header
    let language = req
        .headers()
        .get("accept-language")
        .and_then(|value| value to_str().ok())
        .unwrap_or("Unknown");
    console_debug!("accept language: {:?}", language);

    // Construct and return a response
    let mut headers = Headers::new();
    headers.append("Content-Type", "text/plain");

    let body = "Hello, World!"; // Example response body

    Response::builder()
        .headers(headers)
        .body(body.into())
        .map_err(Into::into)
}
