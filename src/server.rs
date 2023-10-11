use axum::{handler::get, Router};
use std::net::SocketAddr;
use hyper::Server;

#[tokio::main]
async fn metrics_server() {
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/metrics", get(metrics_handler));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Listening on http://{}", addr);
    Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

async fn root_handler() -> String {
    "<a href=\"/metrics\">/metrics</a>".to_string()
}

async fn metrics_handler() -> String {
    // Collect your metrics here
    // This is just a placeholder string, you'd replace this with your actual metrics
    let metrics = "test 123\ntest 456";
    metrics.to_string()
}
