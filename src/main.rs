
mod commands;
use socketioxide::{extract::{Data, SocketRef}, SocketIo};
use axum::{routing::get, response, Router};
use std::fs;

async fn index() -> response::Html<String> {
    let html_content: String = fs::read_to_string("src/terminal.html").expect("Unable to read file");
    response::Html(html_content)
}

#[tokio::main]
async fn main() {
    commands::create_commands();

    let (layer, io) = SocketIo::new_layer();

    io.ns("/", |s: SocketRef| {
        s.on("command", |s: SocketRef, Data::<String>(data)| {
            s.emit("result", format!("Received {}", data)).ok();
        });
    });

    let app: Router = Router::new()
    .route("/", get(index))
    .layer(layer);

    let listener = tokio::net::TcpListener::bind("localhost:8000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}