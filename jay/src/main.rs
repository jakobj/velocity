use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::runtime;

static STATIC_HTML: &str = include_str!("static.html");
static CV_HTML: &str = include_str!("cv.html");
static STYLE_CSS: &str = include_str!("style.css");
static PIC: &'static [u8] = include_bytes!("photo6019329608475848615_crop.jpg");

fn main() {
    let rt = runtime::Builder::new_current_thread()
        .enable_io()
        .build()
        .unwrap();
    rt.block_on(run());
}

async fn run() {
    // 1) generalize to "assets" which are read at build time
    // 2) minimal logging
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(|req| async {
            Ok::<_, Infallible>(handle(req))
        }))
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

fn handle(req: Request<Body>) -> Response<Body> {
    match req.uri().path() {
        "/static.html" => Response::new(STATIC_HTML.into()),
        "/cv.html" => Response::new(CV_HTML.into()),
        "/style.css" => Response::new(STYLE_CSS.into()),
        "/figures/photo6019329608475848615_crop.jpg" => Response::new(PIC.into()),
        _ => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap()
    }
}
