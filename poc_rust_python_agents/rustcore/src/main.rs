use axum::{
    routing::{get, post},
    Router,
    Json,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

mod didcomm;

#[derive(Deserialize)]
struct PackRequest {
    sender_did: String,
    recipient_did: String,
    plaintext: String,
}

#[derive(Serialize)]
struct PackResponse {
    ciphertext: String,
}

async fn pack_endpoint(Json(payload): Json<PackRequest>) -> Json<PackResponse> {
    println!("➡️  Called /didcomm/pack with message from {} to {}", payload.sender_did, payload.recipient_did);
    // Llamamos la fn sync
    let ciphertext = didcomm::pack_message(
        &payload.sender_did,
        &payload.recipient_did,
        &payload.plaintext,
    );
    Json(PackResponse { ciphertext })
}

#[derive(Deserialize)]
struct UnpackRequest {
    ciphertext: String,
}

#[derive(Serialize)]
struct UnpackResponse {
    plaintext: String,
}

async fn unpack_endpoint(Json(payload): Json<UnpackRequest>) -> Json<UnpackResponse> {
    println!("➡️  Called /didcomm/unpack");
    // Llamamos la fn sync
    let plaintext = didcomm::unpack_message(&payload.ciphertext);
    Json(UnpackResponse { plaintext })
}

async fn health() -> impl IntoResponse {
    "ok"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/didcomm/pack", post(pack_endpoint))
        .route("/didcomm/unpack", post(unpack_endpoint))
        .route("/health", get(health));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Rust microservice is running on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
