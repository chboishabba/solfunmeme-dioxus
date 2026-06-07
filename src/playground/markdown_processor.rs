use dioxus::prelude::*;

/// Render markdown content from the DAO pastebin
#[component]
pub fn MarkdownViewer(content: String) -> Element {
    let html = markdown::to_html(&content);
    rsx! {
        div {
            class: "markdown-viewer",
            dangerous_inner_html: "{html}"
        }
    }
}

crate::register_plugin!(
    "markdown",
    "Markdown renderer",
    crate::plugin::PluginCategory::Data,
    "📝",
    || rsx! { div{"plugin"} }
);
