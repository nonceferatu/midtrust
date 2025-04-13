use yew::prelude::*;
use yew::use_effect_with;
use web_sys::HtmlInputElement;
use gloo_net::http::Request;
use gloo_dialogs::alert;
use wasm_bindgen_futures::spawn_local;
use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};

const API_URL: &str = "http://localhost:3000";

#[derive(Clone, Debug, Deserialize, PartialEq)]
struct Deal {
    id: String,
    alice: String,
    bob: String,
    hash_commit: String,
    amount: u64,
    status: String,
}

#[derive(Serialize)]
struct SubmitProofPayload {
    id: String,
    secret: String,
}

#[derive(Deserialize)]
struct DealsResponse {
    deals: Vec<Deal>,
}

#[function_component]
fn App() -> Html {
    let active_tab = use_state(|| "create".to_string());

    let deals = use_state(|| vec![]);
    let selected_deal = use_state(|| None::<Deal>);
    let proof_secret = use_state(|| "".to_string());

    let alice = use_state(|| "".to_string());
    let bob = use_state(|| "".to_string());
    let amount = use_state(|| "50".to_string());
    let secret = use_state(|| "".to_string());

    // Fetch deals on mount
    {
        let deals = deals.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                if let Ok(res) = Request::get(&format!("{}/deals", API_URL))
                    .send().await
                {
                    if let Ok(json) = res.json::<DealsResponse>().await {
                        deals.set(json.deals);
                    }
                }
            });
            || ()
        });
    }

    let handle_submit_deal = {
        let alice = alice.clone();
        let bob = bob.clone();
        let amount = amount.clone();
        let secret = secret.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let payload = serde_json::json!({
                "alice": *alice,
                "bob": *bob,
                "amount": amount.parse::<u64>().unwrap_or(0),
                "secret_hash": format!("{:x}", Sha256::digest(secret.as_bytes())),
            });

            spawn_local(async move {
                if let Ok(req) = Request::post(&format!("{}/deals", API_URL))
                    .header("Content-Type", "application/json")
                    .body(payload.to_string())
                {
                    let _ = req.send().await;
                    alert("✅ Deal created!");
                    web_sys::window().unwrap().location().reload().unwrap();
                }
            });
        })
    };

    let handle_proof_submit = {
        let selected_deal = selected_deal.clone();
        let secret = proof_secret.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if let Some(deal) = &*selected_deal {
                let payload = SubmitProofPayload {
                    id: deal.id.clone(),
                    secret: (*secret).clone(),
                };

                spawn_local(async move {
                    let res = Request::post(&format!("{}/proof", API_URL))
                        .json(&payload).unwrap()
                        .send().await;

                    match res {
                        Ok(resp) if resp.status() == 200 => {
                            let updated: Option<Deal> = resp.json().await.unwrap_or(None);
                            match updated {
                                Some(d) => alert(&format!("✅ Proof verified! Status: {}", d.status)),
                                None => alert("❌ Invalid proof or deal not pending"),
                            }
                            web_sys::window().unwrap().location().reload().unwrap();
                        }
                        _ => alert("⚠️ Error submitting proof."),
                    }
                });
            }
        })
    };

    let tab_class = |name: &str| {
        if *active_tab == name {
            "tab-button active"
        } else {
            "tab-button"
        }
    };

    let switch_tab = {
        let active_tab = active_tab.clone();
        move |name: &'static str| {
            let active_tab = active_tab.clone();
            Callback::from(move |_| active_tab.set(name.to_string()))
        }
    };

    html! {
        <div class="container py-5">
            <div class="text-center mb-4">
            <div class="title-wrapper" style="font-family: 'Poppins', jetbrains-mono;" weight="1000">
                <img src="midtrust_logo.png" alt="MidTrust Logo" style="width: 90px; height: auto;"/>
                <h2 class="m-0">{"MidTrust"}</h2>
            </div>
                <p class="text-muted">{"Simulated ZK with Hash Verification"}</p>
            </div>

            <div class="d-flex justify-content-center mb-4 border-bottom tab-bar">
                <button class={tab_class("create")} onclick={switch_tab("create")}>{"Create Deal"}</button>
                <button class={tab_class("proof")} onclick={switch_tab("proof")}>{"Submit Proof"}</button>
                <button class={tab_class("deals")} onclick={switch_tab("deals")}>{"Deals"}</button>
                <button class={tab_class("about")} onclick={switch_tab("about")}>{"About"}</button>
            </div>

            <div class="glass tab-content">
                {
                    match &**active_tab {
                        "create" => html! {
                            <form class="app-card" onsubmit={handle_submit_deal}>
                                <h5 class="mb-3 text-white text-center">{"Create New Deal"}</h5>
                                <input type="text" placeholder="Sender" value={(*alice).clone()} oninput={input_handler(alice.clone())} />
                                <input type="text" placeholder="Recipient" value={(*bob).clone()} oninput={input_handler(bob.clone())} />
                                <input type="number" placeholder="Amount" value={(*amount).clone()} oninput={input_handler(amount.clone())} />
                                <input type="text" placeholder="Secret" value={(*secret).clone()} oninput={input_handler(secret.clone())} />
                                <button type="submit">{"Create Deal"}</button>
                            </form>
                        },
                        "proof" => html! {
                            <form class="app-card" onsubmit={handle_proof_submit}>
                                <h5 class="mb-3 text-white text-center">{"Submit Proof"}</h5>
                                {
                                    if let Some(deal) = &*selected_deal {
                                        html! { <p>{"Deal ID: "}<strong>{ &deal.id[..8] }</strong></p> }
                                    } else {
                                        html! { <p class="text-muted">{"Select a deal from the Deals tab first."}</p> }
                                    }
                                }
                                <input type="text" placeholder="Enter secret" value={(*proof_secret).clone()} oninput={input_handler(proof_secret.clone())} />
                                <button type="submit">{"Submit Proof"}</button>
                            </form>
                        },
                        "deals" => html! {
                            <div class="app-card">
                                <h5 class="mb-3 text-white text-center">{"Available Deals"}</h5>
                                {
                                    if deals.is_empty() {
                                        html! { <p>{"No deals yet."}</p> }
                                    } else {
                                        deals.iter().map(|d: &Deal| {
                                            let d_id = d.id.clone();
                                            let d_alice = d.alice.clone();
                                            let d_bob = d.bob.clone();
                                            let d_status = d.status.clone();
                                            let d_for_select = d.clone();
                                        
                                            let set_deal = {
                                                let selected_deal = selected_deal.clone();
                                                Callback::from(move |_| selected_deal.set(Some(d_for_select.clone())))
                                            };
                                        
                                            html! {
                                                <div class="p-3 my-2 rounded bg-dark text-white" onclick={set_deal}>
                                                    <strong>{ &d_id[..8] }</strong>{": "} { &d_alice }{" → "}{ &d_bob }
                                                    <span class="ms-2 badge bg-info">{ d_status }</span>
                                                </div>
                                            }
                                        }).collect::<Html>()
                                    }
                                }
                            </div>
                        },
                        "about" => html! {
                            <>
                                <h4 class="mb-3">{"About MidTrust"}</h4>
                                <p>{"MidTrust lets users send crypto in a trustless way, using hash-based zero-knowledge-style verification."}</p>
                            </>
                        },
                        _ => html! { <p>{"Invalid tab"}</p> }
                    }
                }
            </div>
        </div>
    }
}

fn input_handler(state: UseStateHandle<String>) -> Callback<InputEvent> {
    Callback::from(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        state.set(input.value());
    })
}

fn main() {
    yew::Renderer::<App>::new().render();
}