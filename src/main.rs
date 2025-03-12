use std::{fs::File, io::Write};

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
        println!("Got the file!");

        // Grab the name
        let file_name = field.file_name().unwrap();

        // create a path for the soon-to-be file
        let file_path = format!("files/{}", file_name);

        // unwrap the incoming bytes
        let data = field.bytes().await.unwrap();

        // Open a handle to the file
        let mut file_handle = File::create(file_path).expect("Failed to open file handle!");

        // write the incoming data to the handle
        file_handle.write_all(&data).expect("Failed to write data!");
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
