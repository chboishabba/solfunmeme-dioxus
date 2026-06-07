use crate::model::erdfa::{fibonacci_tiers, PasteStatus, TOKEN_CA};
use dioxus::prelude::*;

/// Pastebin view — submit tx data, view submissions, earn bounties
#[component]
pub fn Pastebin() -> Element {
    let mut content = use_signal(|| String::new());
    let mut submissions = use_signal(|| Vec::<serde_json::Value>::new());
    let mut status_msg = use_signal(|| String::new());
    let endpoint = "https://solana.solfunmeme.com/solfunmeme/paste";

    // Fetch existing submissions on mount
    use_effect(move || {
        spawn(async move {
            if let Ok(resp) = reqwasm::http::Request::get(endpoint).send().await {
                if let Ok(text) = resp.text().await {
                    if let Ok(items) = serde_json::from_str::<Vec<serde_json::Value>>(&text) {
                        submissions.set(items);
                    }
                }
            }
        });
    });

    rsx! {
        div { class: "pastebin-view",
            h2 { "📋 Stego Pastebin" }

            div { class: "paste-submit",
                h3 { "Submit Transaction Data" }
                p { "Paste a getTransaction JSON result to earn bounty (0.001 SOL per valid tx)" }
                textarea {
                    class: "paste-input",
                    rows: "8",
                    placeholder: "Paste JSON-RPC getTransaction result here...",
                    value: "{content}",
                    oninput: move |e| content.set(e.value()),
                }
                button {
                    class: "btn-submit",
                    onclick: move |_| {
                        let body = content.read().clone();
                        let ep = endpoint.to_string();
                        spawn(async move {
                            match reqwasm::http::Request::post(&ep)
                                .header("Content-Type", "text/plain")
                                .body(body)
                                .send().await {
                                Ok(resp) => {
                                    if let Ok(text) = resp.text().await {
                                        status_msg.set(format!("✓ Submitted: {}", &text[..80.min(text.len())]));
                                    }
                                }
                                Err(e) => status_msg.set(format!("✗ Error: {e}")),
                            }
                        });
                    },
                    "Submit"
                }
                p { class: "status-msg", "{status_msg}" }
            }

            div { class: "paste-list",
                h3 { "Recent Submissions ({submissions.read().len()})" }
                for item in submissions.read().iter().rev().take(20) {
                    div { class: "paste-item",
                        span { class: "paste-id", "{item[\"id\"].as_str().unwrap_or(\"?\")}" }
                        span { class: "paste-status", " [{item[\"status\"].as_str().unwrap_or(\"?\")}]" }
                        span { class: "paste-time", " {item[\"submitted_at\"].as_str().unwrap_or(\"\")}" }
                    }
                }
            }

            div { class: "bounty-info",
                h3 { "💰 Bounty Status" }
                p { "Token: {TOKEN_CA}" }
                p { "Missing transactions: ~2,532,473" }
                p { "Reward: 0.001 SOL per valid submission" }
                p { "Format: JSON-RPC getTransaction with jsonParsed encoding" }
            }

            div { class: "tier-info",
                h3 { "🏛️ Fibonacci Tiers" }
                table {
                    tr { th { "Tier" } th { "Boundary" } }
                    for (name, boundary) in fibonacci_tiers().iter().take(6) {
                        tr {
                            td { "{name}" }
                            td { "{boundary}" }
                        }
                    }
                }
            }
        }
    }
}

crate::register_plugin!(
    "pastebin",
    "Submit TX data, earn bounties",
    crate::plugin::PluginCategory::Data,
    "📋",
    || rsx! { div{"plugin"} }
);
