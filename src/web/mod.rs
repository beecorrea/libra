mod request;
use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::Mutex;
use warp::Filter;

pub async fn backend(port: u16) {
    let hello = warp::path::end().map(move || format!("Hello from {}", port));

    warp::serve(hello).run(([127, 0, 0, 1], port)).await;
}

pub async fn balancer(port: u16, servers: Arc<Mutex<VecDeque<u16>>>) {
    let routes = warp::path!("fwd").and_then(move || {
        let target = next_server(&servers);
        request::forward(target)
    });
    warp::serve(routes).run(([127, 0, 0, 1], port)).await;
}

fn next_server(servers: &Arc<Mutex<VecDeque<u16>>>) -> u16 {
    let queue_lock = servers.lock();
    let mut queue = queue_lock.unwrap();
    let curr = queue.pop_front().unwrap();
    println!("will forward to {}", curr);
    queue.push_back(curr);
    curr
}
