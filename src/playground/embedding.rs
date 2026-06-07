use crate::model::erdfa::{fibonacci_tiers, TOKEN_CA};
use dioxus::prelude::*;

/// Display token embedding and tier info from erdfa-publish
#[component]
pub fn EmbeddingViewer() -> Element {
    let tiers = fibonacci_tiers();
    rsx! {
        div { class: "embedding-viewer",
            h3 { "🔢 Token Embedding" }
            p { "Mint: {TOKEN_CA}" }
            h4 { "Fibonacci Tiers" }
            for (name, boundary) in tiers.iter() {
                div { "{name}: {boundary}" }
            }
        }
    }
}

crate::register_plugin!(
    "embedding",
    "Token tier viewer",
    crate::plugin::PluginCategory::Data,
    "🔢",
    || rsx! { div{"plugin"} }
);
