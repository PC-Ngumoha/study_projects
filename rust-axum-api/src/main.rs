use axum::{routing::get, Router};
// use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root_handler));

    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // println!("Server up and running at http://{}", addr);

    // axum::serve::bind(&addr)
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!(
        "Server up and running at http://{}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await.unwrap();
}

async fn root_handler() -> &'static str {
    "Hello guys, I'm trying my hands at backend development with Rust"
}
