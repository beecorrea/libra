use std::sync::atomic::AtomicUsize;

use reqwest::{self, Error};
use warp::http::Response;

async fn _forward(port: u16) -> Result<String, Error> {
    let addr = format!("http://127.0.0.1:{}/", port);
    let resp = reqwest::get(addr).await?.text().await?;
    Ok(resp)
}

static NEXT: AtomicUsize = AtomicUsize::new(0);
const SERVERS: [u16; 2] = [3001, 3002];
pub async fn forward() -> Result<impl warp::Reply, warp::Rejection> {
    let curr_server = NEXT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let port = SERVERS[curr_server % 2];

    let fwd = _forward(port).await;
    match fwd {
        Ok(data) => Ok(Response::new(data)),
        Err(_err) => Err(warp::reject::reject()),
    }
}
