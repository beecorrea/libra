mod web;
use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::join;
#[tokio::main]
async fn main() {
    let backends: VecDeque<u16> = VecDeque::from([3001, 3002]);
    let m: Arc<Mutex<VecDeque<u16>>> = Arc::new(Mutex::new(backends));
    join!(
        web::balancer(3000, m),
        web::backend(3001),
        web::backend(3002),
    );
}
