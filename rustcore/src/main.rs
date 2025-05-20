mod didcomm;

use axum::{routing::post, extract::Json, response::IntoResponse, Router};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

/* DTOs */
#[derive(Deserialize)]
struct PackReq  { sender_did:String, recipient_did:String, plaintext:String }
#[derive(Serialize)]
struct PackRes  { ciphertext:String }

#[derive(Deserialize)]
struct UnpackReq{ my_did:String, ciphertext:String }
#[derive(Serialize)]
struct UnpackRes{ plaintext:String }

/* handlers */
async fn pack(Json(r):Json<PackReq>)->impl IntoResponse{
    Json(PackRes{
        ciphertext: didcomm::pack_message(&r.sender_did,&r.recipient_did,&r.plaintext)
    })
}
async fn unpack(Json(r):Json<UnpackReq>)->impl IntoResponse{
    Json(UnpackRes{
        plaintext: didcomm::unpack_message(&r.ciphertext,&r.my_did)
    })
}

/* server */
#[tokio::main]
async fn main(){
    let app = Router::new()
        .route("/didcomm/pack",   post(pack))
        .route("/didcomm/unpack", post(unpack));

    let addr = SocketAddr::from(([127,0,0,1],3000));
    println!("✅ rustcore listening on http://{addr}");
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}
