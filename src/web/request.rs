use reqwest::{self, Error};
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::SeqCst;
use warp::http::Response;

async fn _forward(port: u16) -> Result<String, Error> {
    let addr = format!("http://127.0.0.1:{}/", port);
    let resp = reqwest::get(addr).await?.text().await?;
    Ok(resp)
}

static NEXT: AtomicUsize = AtomicUsize::new(0);
const SERVERS: [u16; 2] = [3001, 3002];
pub async fn forward() -> Result<impl warp::Reply, warp::Rejection> {
    let port = next_server();
    let fwd = _forward(port).await;
    match fwd {
        Ok(data) => Ok(Response::new(data)),
        Err(_err) => Err(warp::reject::reject()),
    }
}

// This overflows after 2^16 requests. Should probably use a queue here.
fn next_server() -> u16 {
    // Update the round-robin table.
    let curr_server = NEXT.fetch_add(1, SeqCst);

    SERVERS[curr_server % 2]
}
