use axum::{Router, response::Html, routing::get};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(index));
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to start Listener!");
    axum::serve(listener, app)
        .await
        .expect("Failed to serve 'app'!"); // This function starts the HTTP server.
}

async fn index() -> Html<&'static str> {
    Html(std::include_str!("../public/index.html"))
}
