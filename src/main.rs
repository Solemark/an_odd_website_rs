mod router;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    router::routes::routing().await;
}
