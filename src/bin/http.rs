use hyper::header::CONTENT_TYPE;
use hyper::http::HeaderValue;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};

use hyper::Method;
use hyper::StatusCode;
use hyper::Error;

use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
struct User {
    id: u32,
    username: String,
    email: String,
}

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
        (&Method::POST, "/user") => {
            // 接收 JSON 请求并返回相同 JSON 数据
            let user_data = hyper::body::to_bytes(req.into_body()).await?;
            let user_result = serde_json::from_slice::<User>(&user_data);
            
            match user_result {
                Ok(user) => {
                    let response_body = serde_json::to_vec(&user).unwrap();
                    
                    let mut response = Response::default();
                    *response.body_mut() = Body::from(response_body);
                    response.headers_mut().insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
                    
                    Ok(response)
                },
                Err(err) => {
                    let error_msg = format!(
                        r#"{{"code": "{}","msg": {}}}"#,
                        400,err
                    );
                    let error_response = Response::builder()
                        .status(400)
                        .body(Body::from(error_msg))
                        .unwrap();
                    Ok(error_response)
                }
            }
        },
        _ => {
            // 未知路由或其他请求处理
            let response = Response::builder()
                .status(404)
                .body(Body::empty())
                .unwrap();
            Ok(response)
        }
    }
}