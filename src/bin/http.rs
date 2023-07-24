use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};

use hyper::Method;
use hyper::StatusCode;
use hyper::Error;


#[tokio::main]
async fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();

    let make_svc = make_service_fn(|_conn| {
        async {
        Ok::<_, Error>(service_fn(move |req| {
            handle_request(req)
        }))
    }
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            // 处理根路由请求
            let res = Response::builder()
                .status(StatusCode::OK)
                .header("Custom-Header","Value")
                .body(Body::from("Hello, World!"))
                .unwrap();
            Ok(res)
        },
        (&Method::GET, "/about") => {
            // 处理 "/about" 路由请求
            let res = Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type","application/json")
                .body(Body::from("{a:1}"))
                .unwrap();
            Ok(res)
        },
        _ => {
            // 处理其他未知路由请求
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap())
        }
    }
}