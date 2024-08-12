use anyhow::Result;
use axum::{extract::Path, response::IntoResponse, routing, Json};
use meme_cache::{footprint, get, set, size};
use serde_json::Value;
use tokio::net::TcpListener;

pub async fn list_all_todos() -> Result<Vec<Value>> {
    let response = reqwest::get("https://jsonplaceholder.typicode.com/todos").await?;
    let json: Vec<Value> = response.json().await.unwrap();
    Ok(json)
}

pub async fn read_one_todo(id: String) -> Result<Value> {
    let response = reqwest::get(format!("https://jsonplaceholder.typicode.com/todos/{id}")).await?;
    let json: Value = response.json().await.unwrap();
    Ok(json)
}

// every 10 seconds, cache expensive api call result and return, otherwise rerun logic
pub async fn list_all_handler() -> impl IntoResponse {
    if let Some(exists) = get::<Vec<Value>>("all_todos").await {
        return Json(exists);
    }
    let json = list_all_todos().await.unwrap();
    set("all_todos", &json, 10_000).await;
    let memory = footprint().await;
    let len = size().await;
    println!("cache memory: {memory} of items: {len}");
    Json(json)
}

// every 10 seconds, cache expensive api call result and return, otherwise rerun logic
pub async fn read_one(id: Path<String>) -> impl IntoResponse {
    let id = id.to_string();
    if let Some(exists) = get::<Value>(&id).await {
        return Json(exists);
    }
    let json = read_one_todo(id.to_string()).await.unwrap();
    set(&id, &json, 10_000).await;
    let memory = footprint().await;
    let len = size().await;
    println!("cache memory: {memory} of items: {len}");
    Json(json)
}

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    let app = axum::Router::new()
        .route("/", routing::get(list_all_handler))
        .route("/:id", routing::get(read_one));
    Ok(axum::serve(listener, app).await?)
}
