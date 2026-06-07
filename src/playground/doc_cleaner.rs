use dioxus::prelude::*;

/// View and clean founding documents
#[component]
pub fn DocCleaner() -> Element {
    let docs = use_signal(|| {
        vec![
            ("README.md", "Project overview"),
            ("CODEBASE_MAP.md", "33K line inventory"),
            ("founding_documents/", "Original vision docs"),
        ]
    });

    rsx! {
        div { class: "doc-cleaner",
            h3 { "📄 Founding Documents" }
            for (name, desc) in docs.read().iter() {
                div { class: "doc-entry",
                    strong { "{name}" }
                    span { " — {desc}" }
                }
            }
        }
    }
}

crate::register_plugin!(
    "doc_cleaner",
    "Founding document browser",
    crate::plugin::PluginCategory::Data,
    "📄",
    || rsx! { div{"plugin"} }
);
