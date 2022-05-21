use warp::{Filter};

#[tokio::main]
async fn main() {
    let hello = warp::path::end().map(|| "Hello, Rust!");

    let port = std::env::var("server.port")
        .unwrap_or("8080".to_string())
        .as_str()
        .parse::<u16>()
        .unwrap();

    println!("Listening on port {}", port);

    warp::serve(hello).run(([0, 0, 0, 0], port)).await;
}
