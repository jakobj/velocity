use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::runtime;

static STR_ASSETS: &'static [(&str, &str)] = &[
    // include your "string" assets (html, css) using `include_str!` in
    // ALPHABETICAL ORDER! first value is the route, second the file path
    ("/static.html", include_str!("../../assets/static.html")),
];

static BYTE_ASSETS: &'static [(&str, &'static [u8])] = &[
    // include your "byte" assets (images, ...) using `include_bytes!` in
    // ALPHABETICAL ORDER! first value is the route, second the file path
    ("/ferris.gif", include_bytes!("../../assets/ferris.gif")),
];

fn main() {
    let rt = runtime::Builder::new_current_thread()
        .enable_io()
        .build()
        .unwrap();
    rt.block_on(run());
}

async fn run() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(|req| async { Ok::<_, Infallible>(handle(req)) }))
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

fn handle(req: Request<Body>) -> Response<Body> {
    if let Ok(index) = STR_ASSETS.binary_search_by_key(&req.uri().path(), |&(a, _b)| a) {
        Response::new(STR_ASSETS[index].1.into())
    } else if let Ok(index) = BYTE_ASSETS.binary_search_by_key(&req.uri().path(), |&(a, _b)| a) {
        Response::new(BYTE_ASSETS[index].1.into())
    } else {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap()
    }
}
