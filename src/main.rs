use futures::TryStreamExt as _;
use hyper::error::Error;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use hyper::{Method, StatusCode};
use nanoid;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize)]
struct Store {
    name: String,
    age: u8,
}

struct UserData {
    id: Option<String>,
    data: Option<Store>,
}

async fn get_query(
    bytes: Vec<u8>,
    my_map: Arc<Mutex<HashMap<String, Store>>>,
) -> Result<Body, Error> {
    println!("get query -> {}", String::from_utf8_lossy(&bytes));
    Ok(Body::empty())
}

async fn insert_data(
    bytes: Vec<u8>,
    my_map: Arc<Mutex<HashMap<String, Store>>>,
) -> Result<Body, Error> {
    println!("insert data -> {}", String::from_utf8_lossy(&bytes));
    Ok(Body::empty())
}

async fn resv_conn(
    req: Request<Body>,
    my_map: Arc<Mutex<HashMap<String, Store>>>,
) -> Result<Response<Body>, Error> {
    let mut response = Response::new(Body::empty());
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/store/query") => {
            let body_bytes: Vec<u8> = hyper::body::to_bytes(req.into_body())
                .await?
                .iter()
                .cloned()
                .collect();
            *response.body_mut() = get_query(body_bytes, my_map).await?;
        }
        (&Method::POST, "/store/insert") => {
            let body_bytes: Vec<u8> = hyper::body::to_bytes(req.into_body())
                .await?
                .iter()
                .cloned()
                .collect();
            *response.body_mut() = insert_data(body_bytes, my_map).await?;
        }
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };

    Ok(response)
}

#[tokio::main]
async fn main() {
    let my_map: HashMap<String, Store> = HashMap::new();
    let mtx_map = Arc::new(Mutex::new(my_map));

    let addr: SocketAddr = "127.0.0.1:8000".parse().unwrap();
    let make_svc = make_service_fn(move |_| {
        let mtx_map_clone = mtx_map.clone();
        async move { Ok::<_, Error>(service_fn(move |req| resv_conn(req, mtx_map_clone.clone()))) }
    });
    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error {}", e);
    }
}
