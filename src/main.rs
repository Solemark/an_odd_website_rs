mod router;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let addr = match std::env::args().nth(1) {
        Some(s) => s,
        None => "localhost".to_string(),
    };
    let port = match std::env::args().nth(2) {
        Some(s) => s.parse().expect("unable to parse argument"),
        None => 8080,
    };
    router::routes::routing(addr, port).await;
}
