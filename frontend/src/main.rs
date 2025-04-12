use yew::prelude::*;
use gloo_net::http::Request;
use gloo_console::log;
use web_sys::HtmlInputElement;
use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};
use gloo_dialogs::alert;
use std::rc::Rc;

// API URL - change this if your backend is on a different port
const API_URL: &str = "http://localhost:3000";

// Deal struct that matches our backend definition
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
struct Deal {
    id: String,
    alice: String,
    bob: String,
    hash_commit: String,
    amount: u64,
    status: String,
}

// Payload for creating a deal
#[derive(Debug, Serialize)]
struct CreateDealPayload {
    alice: String,
    bob: String,
    secret_hash: String,
    amount: u64,
}

// Payload for submitting a proof
#[derive(Debug, Serialize)]
struct SubmitProofPayload {
    id: String,
    secret: String,
}

// Response for listing deals
#[derive(Debug, Deserialize)]
struct DealsResponse {
    deals: Vec<Deal>,
}

// Main app component
#[function_component]
fn App() -> Html {
    // State
    let deals = use_state(|| Vec::<Deal>::new());
    let alice_name = use_state(|| String::from("Alice"));
    let bob_name = use_state(|| String::from("Bob"));
    let amount = use_state(|| 50);
    let secret = use_state(|| String::from("hunter2"));
    let proof_secret = use_state(|| String::new());
    let selected_deal_id = use_state(|| String::new());
    
    // Fetch deals on component mount
    {
        let deals = deals.clone();
        use_effect(move || {
            wasm_bindgen_futures::spawn_local(async move {
                match fetch_deals().await {
                    Ok(fetched_deals) => {
                        deals.set(fetched_deals);
                    },
                    Err(err) => {
                        log!("Error fetching deals:", err);
                    }
                }
            });
            || ()
        });        
    }
    
    // Handler for creating a new deal
    let on_create_deal = {
        let alice_name = alice_name.clone();
        let bob_name = bob_name.clone();
        let secret = secret.clone();
        let amount = amount.clone();
        let deals = deals.clone();
        
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            
            let alice = (*alice_name).clone();
            let bob = (*bob_name).clone();
            let secret_val = (*secret).clone();
            let amount_val = *amount;
            let deals = deals.clone();
            
            // Hash the secret
            let secret_hash = hash_secret(&secret_val);
            
            // Create the payload
            let payload = CreateDealPayload {
                alice,
                bob,
                secret_hash,
                amount: amount_val,
            };
            
            // Send the request
            wasm_bindgen_futures::spawn_local(async move {
                match create_deal(payload).await {
                    Ok(deal) => {
                        log!("Deal created:", &deal.id);
                        alert("Deal created successfully!");
                        
                        // Update the deals list
                        match fetch_deals().await {
                            Ok(fetched_deals) => {
                                deals.set(fetched_deals);
                            },
                            Err(err) => {
                                log!("Error fetching deals:", err);
                            }
                        }
                    },
                    Err(err) => {
                        let msg = err.clone();
                        log!("Error creating deal:", msg);
                        alert(&format!("Error creating deal: {}", err));
                    }
                }
            });
        })
    };
    
    // Handler for submitting a proof
    let on_submit_proof = {
        let proof_secret = proof_secret.clone();
        let selected_deal_id = selected_deal_id.clone();
        let deals = deals.clone();
        
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            
            let secret = (*proof_secret).clone();
            let deal_id = (*selected_deal_id).clone();
            let deals = deals.clone();
            
            if deal_id.is_empty() {
                alert("Please select a deal first");
                return;
            }
            
            if secret.is_empty() {
                alert("Please enter the secret");
                return;
            }
            
            let payload = SubmitProofPayload {
                id: deal_id,
                secret,
            };
            
            wasm_bindgen_futures::spawn_local(async move {
                match submit_proof(payload).await {
                    Ok(Some(deal)) => {
                        log!("Proof verified for deal:", &deal.id);
                        alert("Proof verified successfully! Funds released.");
                        
                        // Update the deals list
                        match fetch_deals().await {
                            Ok(fetched_deals) => {
                                deals.set(fetched_deals);
                            },
                            Err(err) => {
                                log!("Error fetching deals:", err);
                            }
                        }
                    },
                    Ok(None) => {
                        alert("Proof verification failed! The secret is incorrect or deal is already completed.");
                    },
                    Err(err) => {
                        let msg = err.clone();
                        log!("Error creating deal:", msg);
                        alert(&format!("Error creating deal: {}", err));
                    }
                }
            });
        })
    };
    
    // Handle changes to form inputs
    let on_alice_change = {
        let alice_name = alice_name.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            alice_name.set(input.value());
        })
    };
    
    let on_bob_change = {
        let bob_name = bob_name.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            bob_name.set(input.value());
        })
    };
    
    let on_amount_change = {
        let amount = amount.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value().parse::<u64>().unwrap_or(0);
            amount.set(value);
        })
    };
    
    let on_secret_change = {
        let secret = secret.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            secret.set(input.value());
        })
    };
    
    let on_proof_secret_change = {
        let proof_secret = proof_secret.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            proof_secret.set(value);
        })
    };
    
    let on_select_deal = {
        let selected_deal_id = selected_deal_id.clone();
        Callback::from(move |id: String| {
            selected_deal_id.set(id);
        })
    };
    
    html! {
        <div class="container">
            <nav class="navbar navbar-expand-lg navbar-dark bg-primary mb-4 rounded">
                <div class="container-fluid">
                    <a class="navbar-brand" href="#">{"🧠 zkP2P Escrow"}</a>
                    <div class="navbar-text text-white">{"Simulated ZK with Hash Verification"}</div>
                </div>
            </nav>
            
            <div class="row">
                // Create Deal Form
                <div class="col-md-5">
                    <div class="card">
                        <div class="card-header bg-primary text-white">
                            {"Create New Deal"}
                        </div>
                        <div class="card-body">
                            <form onsubmit={on_create_deal}>
                                <div class="mb-3">
                                    <label for="alice" class="form-label">{"Alice (Sender)"}</label>
                                    <input 
                                        type="text"
                                        class="form-control"
                                        id="alice"
                                        value={(*alice_name).clone()}
                                        oninput={Callback::from({
                                            let alice_name = alice_name.clone();
                                            move |e: InputEvent| {
                                                let input: HtmlInputElement = e.target_unchecked_into();
                                                alice_name.set(input.value());
                                            }
                                        })}
                                        required=true
                                    />
                                </div>
                                <div class="mb-3">
                                    <label for="bob" class="form-label">{"Bob (Receiver)"}</label>
                                    <input 
                                        type="text"
                                        class="form-control"
                                        id="bob"
                                        value={(*bob_name).clone()}
                                        oninput={Callback::from({
                                            let bob_name = bob_name.clone();
                                            move |e: InputEvent| {
                                                let input: HtmlInputElement = e.target_unchecked_into();
                                                bob_name.set(input.value());
                                            }
                                        })}
                                        required=true
                                    />
                                </div>
                                <div class="mb-3">
                                <label for="amount" class="form-label">{"Amount"}</label>
                                <input 
                                    type="number"
                                    class="form-control"
                                    id="amount"
                                    value={amount.to_string()}
                                    oninput={Callback::from({
                                        let amount = amount.clone();
                                        move |e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            if let Ok(val) = input.value().parse::<u64>() {
                                                amount.set(val);
                                            }
                                        }
                                    })}
                                    min="1"
                                    required=true
                                />
                                </div>
                                <div class="mb-3">
                                    <label for="secret" class="form-label">{"Secret (for ZK Proof)"}</label>
                                    <input 
                                    type="text"
                                    class="form-control"
                                    id="secret"
                                    value={(*secret).clone()}
                                    oninput={Callback::from({
                                        let secret = secret.clone();
                                        move |e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            secret.set(input.value());
                                        }
                                    })}
                                    required=true
                                />
                                    <div class="form-text">{"This is the secret that will be used to release funds. Keep it safe!"}</div>
                                </div>
                                <button type="submit" class="btn btn-primary">{"Create Deal"}</button>
                            </form>
                        </div>
                    </div>
                    
                    // Submit Proof Form
                    <div class="card mt-4">
                        <div class="card-header bg-success text-white">
                            {"Verify Proof & Release Funds"}
                        </div>
                        <div class="card-body">
                            <form onsubmit={on_submit_proof}>
                                <div class="mb-3">
                                    <label for="dealId" class="form-label">{"Selected Deal"}</label>
                                    <input 
                                        type="text" 
                                        class="form-control" 
                                        id="dealId" 
                                        value={(*selected_deal_id).clone()} 
                                        disabled=true
                                    />
                                    <div class="form-text">{"Select a pending deal from the list"}</div>
                                </div>
                                <div class="mb-3">
                                    <label for="proofSecret" class="form-label">{"Secret"}</label>
                                    <input 
                                    type="text"
                                    class="form-control"
                                    id="proofSecret"
                                    value={(*proof_secret).clone()}
                                    oninput={Callback::from({
                                        let proof_secret = proof_secret.clone();
                                        move |e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            proof_secret.set(input.value());
                                        }
                                    })}
                                    required=true
                                />

                                </div>
                                <button type="submit" class="btn btn-success">{"Submit Proof"}</button>
                            </form>
                        </div>
                    </div>
                </div>
                
                // Deals List
                <div class="col-md-7">
                    <div class="card">
                        <div class="card-header bg-info text-white">
                            {"Active Deals"}
                        </div>
                        <div class="card-body">
                            <div class="list-group">
                                {
                                    if deals.is_empty() {
                                        html! {
                                            <div class="alert alert-info">
                                                {"No deals found. Create a new deal to get started."}
                                            </div>
                                        }
                                    } else {
                                        deals.iter().map(|deal| {
                                            let deal = deal.clone();
                                            let id = deal.id.clone();
                                            let on_select = {
                                                let id = id.clone();
                                                let on_select_deal = on_select_deal.clone();
                                                Callback::from(move |_| {
                                                    on_select_deal.emit(id.clone());
                                                })
                                            };
                                            
                                            let status_class = match deal.status.as_str() {
                                                "Pending" => "bg-warning",
                                                "Released" => "bg-success",
                                                _ => "bg-secondary",
                                            };
                                            
                                            html! {
                                                <div 
                                                    class={classes!("list-group-item", "list-group-item-action", 
                                                        if &*selected_deal_id == &deal.id { "active" } else { "" })}
                                                    onclick={on_select}
                                                >
                                                    <div class="d-flex w-100 justify-content-between">
                                                        <h5 class="mb-1">{format!("Deal: {}", &deal.id[0..8])}</h5>
                                                        <span class={classes!("badge", status_class)}>{&deal.status}</span>
                                                    </div>
                                                    <p class="mb-1">{format!("From {} to {}", &deal.alice, &deal.bob)}</p>
                                                    <small>{format!("Amount: {}", deal.amount)}</small>
                                                </div>
                                            }
                                        }).collect::<Html>()
                                    }
                                }
                            </div>
                        </div>
                    </div>
                    
                    // How it Works
                    <div class="card mt-4">
                        <div class="card-header bg-dark text-white">
                            {"How zkP2P Works"}
                        </div>
                        <div class="card-body">
                            <ol>
                                <li><strong>{"Create Deal:"}</strong> {" Alice creates a deal with a secret. The secret is hashed client-side."}</li>
                                <li><strong>{"Lock Funds:"}</strong> {" In a real system, funds would be locked in a smart contract."}</li>
                                <li><strong>{"Submit Proof:"}</strong> {" Bob submits the secret, proving they know it."}</li>
                                <li><strong>{"Verify Proof:"}</strong> {" The system verifies the hash of the secret matches."}</li>
                                <li><strong>{"Release Funds:"}</strong> {" When verified, funds are released to Bob."}</li>
                            </ol>
                            <div class="alert alert-info">
                                <strong>{"Note:"}</strong> {" In a real system, this would use real Zero-Knowledge Proofs. Here we simulate with hashes."}
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

// Hash a secret using SHA-256
fn hash_secret(secret: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(secret.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}

// API Functions

// Fetch all deals
async fn fetch_deals() -> Result<Vec<Deal>, String> {
    match Request::get(&format!("{}/deals", API_URL))
        .send()
        .await {
            Ok(response) => {
                if response.status() == 200 {
                    let result = response.json::<DealsResponse>().await;
                    match result {
                        Ok(data) => Ok(data.deals),
                        Err(err) => Err(format!("Failed to parse response: {}", err))
                    }
                } else {
                    Err(format!("Server error: {}", response.status()))
                }
            },
            Err(err) => Err(format!("Network error: {}", err))
        }
}

// Create a new deal
async fn create_deal(payload: CreateDealPayload) -> Result<Deal, String> {
    match Request::post(&format!("{}/deals", API_URL))
        .json(&payload)
        .unwrap()
        .send()
        .await {
            Ok(response) => {
                if response.status() == 200 {
                    let result = response.json::<Deal>().await;
                    match result {
                        Ok(deal) => Ok(deal),
                        Err(err) => Err(format!("Failed to parse response: {}", err))
                    }
                } else {
                    Err(format!("Server error: {}", response.status()))
                }
            },
            Err(err) => Err(format!("Network error: {}", err))
        }
}

// Submit a proof
async fn submit_proof(payload: SubmitProofPayload) -> Result<Option<Deal>, String> {
    match Request::post(&format!("{}/proof", API_URL))
        .json(&payload)
        .unwrap()
        .send()
        .await {
            Ok(response) => {
                if response.status() == 200 {
                    let result = response.json::<Option<Deal>>().await;
                    match result {
                        Ok(deal) => Ok(deal),
                        Err(err) => Err(format!("Failed to parse response: {}", err))
                    }
                } else {
                    Err(format!("Server error: {}", response.status()))
                }
            },
            Err(err) => Err(format!("Network error: {}", err))
        }
}

// Main function
fn main() {
    yew::Renderer::<App>::new().render();
}