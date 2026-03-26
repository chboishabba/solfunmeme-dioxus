use crate::extractor::types::CodeSnippet;
use dioxus::prelude::*;
use std::collections::HashSet;
use tracing::info;
pub fn copy_all_snippets_combined(
    snippets: Vec<CodeSnippet>,
    copied_snippets: Signal<HashSet<String>>,
) {
    let combined = snippets
        .iter()
        .map(|snippet| snippet.content.clone())
        .collect::<Vec<_>>()
        .join("\n\n");

    copy_to_clipboard(combined, "all_snippets".to_string(), copied_snippets);
}

pub fn copy_to_clipboard(
    text: String,
    snippet_id: String,
    mut copied_snippets: Signal<HashSet<String>>,
) {
    info!("going to save: {:?}", text);

    #[cfg(target_arch = "wasm32")]
    {
        wasm_bindgen_futures::spawn_local(async move {
            let window = web_sys::window().expect("window");
            let nav = window.navigator().clipboard();
            let promise = nav.write_text(&text);
            match wasm_bindgen_futures::JsFuture::from(promise).await {
                Ok(_) => info!("clipboard write succeeded"),
                Err(error) => info!("clipboard write failed: {:?}", error),
            }
        });
    }

    copied_snippets.write().insert(snippet_id.clone());
    spawn(async move {
        #[cfg(target_arch = "wasm32")]
        gloo_timers::future::TimeoutFuture::new(2_000).await;
        #[cfg(not(target_arch = "wasm32"))]
        tokio::time::sleep(std::time::Duration::from_millis(2_000)).await;
        copied_snippets.write().remove(&snippet_id);
    });
}

// pub fn copy_to_clipboard(text: String, snippet_id: String, mut copied_snippets: Signal<HashSet<String>>) {

pub fn create_copy_handler(
    copied_snippets: Signal<HashSet<String>>,
) -> impl FnMut(String, String) + Clone {
    move |text: String, snippet_id: String| {
        copy_to_clipboard(text, snippet_id, copied_snippets.clone());
    }
}
