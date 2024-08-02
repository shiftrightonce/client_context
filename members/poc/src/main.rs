use std::{fmt::Display, sync::Arc};

use axum::{extract::Path, response::Html, routing::get, Router};
use client_context::ContextManager;

#[tokio::main]
async fn main() {
    let mut app = Router::new();
    let manager = Arc::new(ContextManager::new());

    app = app.route("/", get(|| async { Html("User context POC application") }));
    app = app.route(
        "/client/:id",
        get(|Path(id): Path<String>| async move {
            let m = Arc::clone(&manager);
            let client = m
                .context(&id, 10, || {
                    Box::pin({
                        let name = id.clone();
                        async move { Client::new(&name) }
                    })
                })
                .await;

            Html(format!("welcome client: {}", client))
        }),
    );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("running on port: 3000 ");
    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, Clone)]
struct Client {
    id: i32,
    name: String,
}

impl Client {
    pub fn new(name: &str) -> Self {
        Self {
            id: 0,
            name: name.to_string(),
        }
    }
}

impl Display for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id: {}, name: {}", self.id, &self.name)
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        println!("dropping client: {}", &self.name);
    }
}
