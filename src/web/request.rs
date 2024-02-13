use reqwest::{self, Error};
use warp::http::Response;

async fn _forward(port: u16) -> Result<String, Error> {
    let addr = format!("http://127.0.0.1:{}/", port);
    let resp = reqwest::get(addr).await?.text().await?;
    Ok(resp)
}

pub async fn forward(port: u16) -> Result<impl warp::Reply, warp::Rejection> {
    let fwd = _forward(port).await;
    match fwd {
        Ok(data) => Ok(Response::new(data)),
        Err(_err) => Err(warp::reject::reject()),
    }
}
