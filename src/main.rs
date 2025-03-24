// use std::{fs::File, io::Write};

// use axum::{Router, extract::Multipart, response::Html, routing::get};

// async fn index() -> Html<&'static str> {
//     Html(std::include_str!("../public/index.html"))
// }

// async fn upload(mut multipart: Multipart) {
//     while let Some(field) = multipart
//         .next_field()
//         .await
//         .expect("Failed to get next field!")
//     {
//         // if field.name().unwrap() != "fileupload" {
//         //     continue;
//         // }
//         println!("Got the file!");

//         // Grab the name
//         let file_name = field.file_name().unwrap();

//         // create a path for the soon-to-be file
//         let file_path = format!("files/{}", file_name);

//         // unwrap the incoming bytes
//         let data = field.bytes().await.unwrap();

//         // Open a handle to the file
//         let mut file_handle = File::create(file_path).expect("Failed to open file handle!");

//         // write the incoming data to the handle
//         file_handle.write_all(&data).expect("Failed to write data!");
//     }
// }

// #[tokio::main]
// async fn main() {
//     fn main() {
//         let result: Result<i32, &str> = Err("Something went wrong");

//         if let Err(e) = result {
//             println!("Error: {}", e);
//         }
//     }

//     // build our application with a single route
//     let app = Router::new().route("/", get(index).post(upload));
//     // run our app with hyper, listening globally on port 3000
//     let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
//         .await
//         .expect("Failed to start Listener!");
//     axum::serve(listener, app)
//         .await
//         .expect("Failed to serve 'app'!"); // This function starts the HTTP server.
// }

use axum::{
    Router,
    extract::Multipart,
    response::{Html, IntoResponse},
    routing::{get, post},
};
use std::{fs::File, io::Write};
use tokio::fs;

async fn index() -> Html<&'static str> {
    Html(std::include_str!("../public/index.html"))
}

async fn upload(mut multipart: Multipart) -> impl IntoResponse {
    let mut file_paths = Vec::new(); // Vector to hold the paths of uploaded files

    while let Some(field) = multipart
        .next_field()
        .await
        .expect("Failed to get next field!")
    {
        // Skip the non-file fields if needed (e.g., for handling other inputs)
        // if field.name().unwrap() != "fileupload" {
        //     continue;
        // }

        // Grab the file name
        let file_name = field
            .file_name()
            .unwrap_or_else(|| "default_filename".into());

        // Create a path for the soon-to-be file in the 'files' directory
        let file_path = format!("files/{}", file_name);

        // Unwrap the incoming bytes
        let data = field.bytes().await.unwrap();

        // Create the file on the disk
        let mut file_handle = File::create(&file_path).expect("Failed to open file handle!");

        // Write the incoming data to the file
        file_handle.write_all(&data).expect("Failed to write data!");

        // Add the file path to the vector for response
        file_paths.push(file_path);
    }

    // Respond with the paths of the uploaded files
    format!("Files uploaded successfully: {:?}", file_paths)
}

#[tokio::main]
async fn main() {
    // Build our application with a route to serve the index.html page and another for file upload
    let app = Router::new()
        .route("/", get(index))
        .route("/upload", post(upload));

    // Run the application with a TCP listener on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to start listener!");

    axum::serve(listener, app)
        .await
        .expect("Failed to serve the app!");
}
