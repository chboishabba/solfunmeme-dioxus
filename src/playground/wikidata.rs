use dioxus::prelude::*;

/// Wikidata concept browser for DAO governance terms
#[component]
pub fn WikidataViewer() -> Element {
    let concepts = vec![
        ("Q7278", "Political party", "DAO governance model"),
        ("Q35127", "Website", "Dioxus WASM frontend"),
        ("Q8142", "Currency", "SOLFUNMEME token"),
        ("Q11639", "Cryptography", "ZK proofs + stego"),
        ("Q131723", "Voting", "Bicameral resolution"),
    ];

    rsx! {
        div { class: "wikidata-viewer",
            h3 { "🌐 Wikidata Concepts" }
            table {
                tr { th { "QID" } th { "Concept" } th { "Mapping" } }
                for (qid, concept, mapping) in concepts.iter() {
                    tr {
                        td { a { href: "https://www.wikidata.org/wiki/{qid}", "{qid}" } }
                        td { "{concept}" }
                        td { "{mapping}" }
                    }
                }
            }
        }
    }
}

crate::register_plugin!(
    "wikidata",
    "Wikidata concept browser",
    crate::plugin::PluginCategory::Data,
    "🌐",
    || rsx! { div{"plugin"} }
);
