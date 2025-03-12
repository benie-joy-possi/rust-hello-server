use axum::{Router, extract::Multipart, response::Html, routing::get};

async fn index() -> Html<&'static str> {
    Html(std::include_str!("../public/index.html"))
}

async fn upload(mut multipart: Multipart) {
    while let Some(field) = multipart
        .next_field()
        .await
        .expect("Failed to get next field!")
    {
        if field.name().unwrap() != "fileupload" {
            continue;
        }
        println!("Got the file!")
    }
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(index).post(upload));
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to start Listener!");
    axum::serve(listener, app)
        .await
        .expect("Failed to serve 'app'!"); // This function starts the HTTP server.
}
