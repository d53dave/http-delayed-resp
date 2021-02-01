use std::{convert::Infallible, net::SocketAddr, env};
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use tokio::time::{sleep, Duration};
use std::collections::HashMap;

async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let params: HashMap<String, String> = req.uri().query().map(|v| {
        url::form_urlencoded::parse(v.as_bytes())
            .into_owned()
            .collect()
    })
    .unwrap_or_else(HashMap::new);

    let delay: u64 = if let Some(n) = params.get("delay") {
                n.parse().unwrap()
            } else {
                1000
            };
    sleep(Duration::from_millis(delay)).await;
    Ok(Response::new("".into()))
}

#[tokio::main]
async fn main() {
    let port: u16 = match env::var("DELAYED_RESP_PORT") {
        Ok(val) => val.parse().unwrap(),
        Err(_e) => 3000
    };

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handle))
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("http-delayed-resp listening on port {}", port);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}