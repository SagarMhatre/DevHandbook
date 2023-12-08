use axum::{
    routing::{get},
    Router
};
use tower_http::add_extension::AddExtensionLayer;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use axum::extract::Extension;
use axum::response::Json;
use axum::response::IntoResponse;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    id: u32,
    name: String,
    age: u32,
}

#[tokio::main]
async fn main() {
    let users = Arc::new(Mutex::new(vec![
        User {
            id: 1,
            name: "John".to_string(),
            age: 25,
        },
        User {
            id: 2,
            name: "Jane".to_string(),
            age: 30,
        },
    ]));

    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // Add users routes
        .route("/users", get(get_users))
        .layer(AddExtensionLayer::new(users));

 
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"         // http://localhost:3000/
}

// http://localhost:3000/users
async fn get_users(users: Extension<Arc<Mutex<Vec<User>>>>) -> impl IntoResponse {
    let users = users.lock().unwrap();
    Json(users.clone())
}