mod web;
use tokio::join;

#[tokio::main]
async fn main() {
    join!(web::balancer(3000), web::backend(3001), web::backend(3002),);
}
