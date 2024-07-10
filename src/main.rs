use std::net::SocketAddr;

mod handlers;
mod router;

#[tokio::main]
async fn main() {
    let app = router::new();
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
