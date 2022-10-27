use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::runtime;

static STATIC_HTML: &str = include_str!("static.html");

fn main() {
    let rt = runtime::Builder::new_current_thread()
        .enable_io()
        .build()
        .unwrap();
    rt.block_on(run());
}

async fn run() {
    // 1) routing
    // 2) include css
    // 3) include picture
    // 4) generalize to "assets" which are read at build time
    // 5) minimal logging
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(static_html))
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn static_html(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(STATIC_HTML.into()))
}
