use dioxus::html::FileData;
use dioxus::{html::HasFileData, prelude::*};

#[component]
pub fn DropZone(hovered: Signal<bool>, read_files: EventHandler<Vec<FileData>>) -> Element {
    rsx! {
        div {
            class: if hovered() { "upload-area drag-over" } else { "upload-area" },
            ondragover: move |evt| {
                evt.prevent_default();
                hovered.set(true);
            },
            ondragleave: move |_| hovered.set(false),
            ondrop: move |evt| async move {
                evt.prevent_default();
                hovered.set(false);
                let selected_files = evt.files();
                if !selected_files.is_empty() {
                    read_files.call(selected_files);
                }
            },
            "🎯 Drop markdown files here or click above to select"
        }
    }
}
