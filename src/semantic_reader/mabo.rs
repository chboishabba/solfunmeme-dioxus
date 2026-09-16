use dioxus::prelude::*;
use sensiblaw_reader_model::PropositionPayment;

use super::SemanticReader;

/// Production Mabo reader shell.
///
/// A caller must supply the typed SLR `PropositionPayment`. Absence is rendered
/// explicitly; this component never fabricates a fixture payment from UI state.
#[component]
pub fn MaboSemanticReader(payment: Option<PropositionPayment>) -> Element {
    match payment {
        Some(payment) => rsx!(SemanticReader { payment }),
        None => rsx!(section {
            class: "semantic-reader semantic-reader-unavailable",
            h2 { "Mabo Semantic Reader" }
            p { "No typed SLR proposition payment is available for this reader session." }
            p { "Source and Why? actions remain unavailable until the Rust payment boundary supplies one." }
        }),
    }
}
