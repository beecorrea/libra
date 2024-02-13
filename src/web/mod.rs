mod request;

use warp::Filter;

pub async fn backend(port: u16) {
    let hello = warp::path::end().map(move || format!("Hello from {}", port));

    warp::serve(hello).run(([127, 0, 0, 1], port)).await;
}

pub async fn balancer(port: u16) {
    let routes = warp::path!("server" / u16).and_then(|p: u16| request::forward(p));
    warp::serve(routes).run(([127, 0, 0, 1], port)).await;
}
