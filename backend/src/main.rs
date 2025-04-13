use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Deal {
    id: String,
    alice: String,
    bob: String,
    hash_commit: String,
    amount: u64,
    status: String, // "Pending", "Released"
}

#[derive(Debug, Deserialize)]
struct CreateDealRequest {
    alice: String,
    bob: String,
    secret_hash: String,
    amount: u64,
}

#[derive(Debug, Deserialize)]
struct SubmitProofRequest {
    id: String,
    secret: String,
}

#[derive(Serialize)]
struct DealsResponse {
    deals: Vec<Deal>,
}

type DealsState = Arc<Mutex<Vec<Deal>>>;

async fn create_deal(data: web::Json<CreateDealRequest>, deals: web::Data<DealsState>) -> impl Responder {
    let new_id = Uuid::new_v4().to_string();
    let mut deals = deals.lock().unwrap();

    let deal = Deal {
        id: new_id.clone(),
        alice: data.alice.clone(),
        bob: data.bob.clone(),
        hash_commit: data.secret_hash.clone(),
        amount: data.amount,
        status: "Pending".to_string(),
    };

    println!("🔐 Created deal {} between {} → {} for {} tokens", new_id, deal.alice, deal.bob, deal.amount);

    deals.push(deal.clone());
    HttpResponse::Ok().json(deal)
}

async fn submit_proof(data: web::Json<SubmitProofRequest>, deals: web::Data<DealsState>) -> impl Responder {
    let mut deals = deals.lock().unwrap();

    if let Some(deal) = deals.iter_mut().find(|d| d.id == data.id) {
        let mut hasher = Sha256::new();
        hasher.update(data.secret.as_bytes());
        let hash = hasher.finalize();
        let computed_hash = format!("{:x}", hash);

        if deal.status == "Pending" && computed_hash == deal.hash_commit {
            println!("✅ Proof accepted for deal {}", deal.id);
            deal.status = "Released".to_string();
            HttpResponse::Ok().json(Some(deal.clone()))
        } else {
            println!("❌ Proof rejected for deal {}", deal.id);
            HttpResponse::Ok().json(None::<Deal>)
        }
    } else {
        println!("⚠️ Deal not found for ID: {}", data.id);
        HttpResponse::NotFound().body("Deal not found")
    }
}

async fn list_deals(deals: web::Data<DealsState>) -> impl Responder {
    let deals = deals.lock().unwrap();
    HttpResponse::Ok().json(DealsResponse {
        deals: deals.clone(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let deals: DealsState = Arc::new(Mutex::new(Vec::new()));
    println!("🚀 zkP2P backend running at http://localhost:3000");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(deals.clone()))
            .wrap(Cors::permissive())
            .route("/deals", web::post().to(create_deal))
            .route("/proof", web::post().to(submit_proof))
            .route("/deals", web::get().to(list_deals))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
