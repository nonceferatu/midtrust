use axum::{
    routing::{get, post},
    http::Method,
    Json, Router, extract::State,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use sha2::{Sha256, Digest};
use tower_http::cors::{CorsLayer, Any};
use uuid::Uuid;

// Deal data structure
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Deal {
    id: String,
    alice: String,
    bob: String,
    hash_commit: String,
    amount: u64,
    status: String, // "Pending", "Locked", "Released", "Cancelled"
}

// Database type - in-memory storage
type Db = Arc<Mutex<HashMap<String, Deal>>>;

// Request payload for creating a deal
#[derive(Debug, Deserialize)]
struct CreateDealPayload {
    alice: String,
    bob: String,
    secret_hash: String, // Pre-hashed secret
    amount: u64,
}

// Request payload for submitting a proof
#[derive(Debug, Deserialize)]
struct SubmitProofPayload {
    id: String,
    secret: String,
}

// Response for listing deals
#[derive(Debug, Serialize)]
struct DealsResponse {
    deals: Vec<Deal>,
}

#[tokio::main]
async fn main() {
    // Initialize in-memory database
    let db: Db = Arc::new(Mutex::new(HashMap::new()));
    
    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);
    
    // Build router with routes
    let app = Router::new()
        .route("/deals", get(list_deals))
        .route("/deals", post(create_deal))
        .route("/proof", post(submit_proof))
        .with_state(db)
        .layer(cors);

    // Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Backend running at http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Handler to list all deals
async fn list_deals(State(db): State<Db>) -> Json<DealsResponse> {
    let deals = db.lock().unwrap().values().cloned().collect();
    Json(DealsResponse { deals })
}

// Handler to create a new deal
async fn create_deal(
    State(db): State<Db>,
    Json(payload): Json<CreateDealPayload>,
) -> Json<Deal> {
    let deal = Deal {
        id: Uuid::new_v4().to_string(),
        alice: payload.alice,
        bob: payload.bob,
        hash_commit: payload.secret_hash,
        amount: payload.amount,
        status: "Pending".to_string(),
    };

    println!("🔐 Deal created: {}", deal.id);
    println!("   → Alice: {}", deal.alice);
    println!("   → Bob: {}", deal.bob);
    println!("   → Amount: {}", deal.amount);
    println!("   → Status: {}", deal.status);
    
    // Store the deal in our database
    db.lock().unwrap().insert(deal.id.clone(), deal.clone());

    Json(deal)
}

// Handler to submit a proof and verify it
async fn submit_proof(
    State(db): State<Db>,
    Json(payload): Json<SubmitProofPayload>,
) -> Json<Option<Deal>> {
    let mut db = db.lock().unwrap();

    // Find the deal by ID
    if let Some(deal) = db.get_mut(&payload.id) {
        // Compute hash of the provided secret
        let mut hasher = Sha256::new();
        hasher.update(payload.secret.as_bytes());
        let computed_hash = format!("{:x}", hasher.finalize());

        println!("🔍 Verifying proof for deal: {}", deal.id);
        println!("   → Stored hash: {}", deal.hash_commit);
        println!("   → Computed hash: {}", computed_hash);
        
        // Compare hashes - this is our simulated "ZK proof verification"
        if deal.status == "Pending" && computed_hash == deal.hash_commit {
            // In a real ZK system, we would verify a proof here
            println!("✅ Proof verified! Release funds");
            deal.status = "Released".to_string();
            return Json(Some(deal.clone()));
        } else {
            println!("❌ Proof invalid or deal not pending");
        }
    } else {
        println!("❌ Deal not found: {}", payload.id);
    }

    Json(None)
}