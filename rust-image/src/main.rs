use warp::{Filter};

#[tokio::main]
async fn main() {
    let hello = warp::path!("" / String)
    .map(|_name| format!("Hello world!"));

    let port = std::env::var("server.port")
    .unwrap_or("8080".to_string()).as_str()
    .parse::<u16>().unwrap();

    warp::serve(hello)
    .run(([127, 0, 0, 1], port))
    .await;
}
