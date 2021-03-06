use futures::TryStreamExt as _;
use hyper::error::Error;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use hyper::{Method, StatusCode};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
//use std::error::Error;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

mod manager;
use manager::Manager;

mod algorithms;
mod graph;

//async fn get_query(
//    bytes: &[u8],
//    my_map: Arc<Mutex<HashMap<String, Option<Store>>>>,
//) -> Result<Body, Error> {
//    let user_data: UserData = serde_json::from_slice(bytes).unwrap();
//    let lock = my_map.lock().unwrap();
//    match user_data.id {
//        Some(id) => match lock.get(&id) {
//            Some(val) => {
//                let json_val = serde_json::to_string(val).unwrap();
//                return Ok(Body::from(json_val));
//            }
//            None => return Ok(Body::from("user identifier not in table")),
//        },
//        None => return Ok(Body::from("no user identifier supplied")),
//    }
//}
//
//async fn insert_data(
//    bytes: &[u8],
//    my_map: Arc<Mutex<HashMap<String, Option<Store>>>>,
//) -> Result<Body, Error> {
//    let user_data: UserData = serde_json::from_slice(bytes).unwrap();
//    let mut lock = my_map.lock().unwrap();
//    match user_data.id {
//        Some(id) => match lock.get(&id) {
//            Some(_) => {
//                lock.insert(id, user_data.data);
//                return Ok(Body::from("Store inserted succsessfuly"));
//            }
//            None => return Ok(Body::from("user identifier not in table")),
//        },
//        None => {
//            let user_id = assign_id();
//            lock.insert(user_id.clone(), user_data.data);
//            return Ok(Body::from(format!(
//                "user id = {} \nStore inserted succsessfuly",
//                user_id
//            )));
//        }
//    }
//}

async fn resv_conn(
    req: Request<Body>,
    //    my_map: Arc<Mutex<HashMap<String, Option<Store>>>>,
    manager: Arc<Mutex<Manager>>,
) -> Result<Response<Body>, Error> {
    let mut response = Response::new(Body::empty());
    match (req.method(), req.uri().path()) {
        //(&Method::POST, "/store/query") => {
        //    let body_bytes: Vec<u8> = hyper::body::to_bytes(req.into_body())
        //        .await?
        //        .iter()
        //        .cloned()
        //        .collect();
        //    *response.body_mut() = get_query(&body_bytes, my_map).await?;
        //}
        //(&Method::POST, "/store/insert") => {
        //    let body_bytes: Vec<u8> = hyper::body::to_bytes(req.into_body())
        //        .await?
        //        .iter()
        //        .cloned()
        //        .collect();
        //    *response.body_mut() = insert_data(&body_bytes, my_map).await?;
        //}
        (&Method::POST, "/create/simple/node_list") => {
            let body_bytes: Vec<u8> = hyper::body::to_bytes(req.into_body())
                .await?
                .iter()
                .cloned()
                .collect();
            let mut lock = manager.lock().unwrap();
            let res = lock.create_simple_from_nodes(body_bytes).unwrap();
            *response.body_mut() = Body::from(res)
        }
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };

    Ok(response)
}

#[tokio::main]
async fn main() {
    //let my_map: HashMap<String, Option<Store>> = HashMap::new();
    //let mtx_map = Arc::new(Mutex::new(my_map));

    let manager = Manager::new();
    let mtx_manager = Arc::new(Mutex::new(manager));

    let addr: SocketAddr = "127.0.0.1:8000".parse().unwrap();
    let make_svc = make_service_fn(move |_| {
        let mtx_manager_clone = mtx_manager.clone();
        async move {
            Ok::<_, Error>(service_fn(move |req| {
                resv_conn(req, mtx_manager_clone.clone())
            }))
        }
    });
    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error {}", e);
    }
}
