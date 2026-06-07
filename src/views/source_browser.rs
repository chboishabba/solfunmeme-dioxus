use crate::embedself::*;
use crate::views::source_browser_style;
use dioxus::prelude::*;

//use source_browser_style;

#[component]
pub fn SourceBrowser() -> Element {
    let mut selected_module = use_signal(|| "src".to_string());
    let mut selected_file = use_signal(|| None::<String>);

    let modules = vec![
        ("src", "Main Source"),
        ("src/bin", "Binaries"),
        ("src/extractor", "Extractor"),
        ("src/extractor/components", "Extractor Components"),
        ("src/extractor/model", "Extractor Model"),
        ("src/extractor/system", "Extractor System"),
        ("src/model", "Model"),
        ("src/model/git", "Git Model"),
        ("src/model/lean", "Lean Model"),
        ("src/model/lean/types", "Lean Types"),
        ("src/model/math", "Math Model"),
        ("src/playground", "Playground"),
        ("src/state", "State"),
        ("src/views", "Views"),
        ("src/views/component_memes", "Component Memes"),
        ("src/views/crypto_frontend", "Crypto Frontend"),
        ("src/views/extras_views", "Extra Views"),
        ("src/views/wikidata_memes", "Wikidata Memes"),
        ("src/views/workflow_memes", "Workflow Memes"),
    ];

    let files = match selected_module().as_str() {
        "src" => OurSource::iter().collect::<Vec<_>>(),
        "src/bin" => OurSourceBin::iter().collect::<Vec<_>>(),
        "src/extractor" => OurSourceExtractor::iter().collect::<Vec<_>>(),
        "src/extractor/components" => OurSourceExtractorComponents::iter().collect::<Vec<_>>(),
        "src/extractor/model" => OurSourceExtractorModel::iter().collect::<Vec<_>>(),
        "src/extractor/system" => OurSourceExtractorSystem::iter().collect::<Vec<_>>(),
        "src/model" => OurSourceModel::iter().collect::<Vec<_>>(),
        "src/model/git" => OurSourceModelGit::iter().collect::<Vec<_>>(),
        "src/model/lean" => OurSourceModelLean::iter().collect::<Vec<_>>(),
        "src/model/lean/types" => OurSourceModelLeanTypes::iter().collect::<Vec<_>>(),
        "src/model/math" => OurSourceModeMath::iter().collect::<Vec<_>>(),
        "src/playground" => OurSourcePlayground::iter().collect::<Vec<_>>(),
        "src/state" => OurSourceState::iter().collect::<Vec<_>>(),
        "src/views" => OurSourceView::iter().collect::<Vec<_>>(),
        "src/views/component_memes" => OurSourceViewComponent::iter().collect::<Vec<_>>(),
        "src/views/crypto_frontend" => OurSourceViewCrypto::iter().collect::<Vec<_>>(),
        "src/views/extras_views" => OurSourceViewextra::iter().collect::<Vec<_>>(),
        "src/views/wikidata_memes" => OurSourceViewWikwidata::iter().collect::<Vec<_>>(),
        "src/views/workflow_memes" => OurSourceViewWorkflow::iter().collect::<Vec<_>>(),
        _ => vec![],
    };

    let file_content = selected_file().and_then(|filename| match selected_module().as_str() {
        "src" => OurSource::get(&filename).map(|f| {
            std::str::from_utf8(&f.data)
                .unwrap_or("Binary file")
                .to_string()
        }),
        "src/bin" => OurSourceBin::get(&filename).map(|f| {
            std::str::from_utf8(&f.data)
                .unwrap_or("Binary file")
                .to_string()
        }),
        "src/extractor" => OurSourceExtractor::get(&filename).map(|f| {
            std::str::from_utf8(&f.data)
                .unwrap_or("Binary file")
                .to_string()
        }),
        "src/extractor/components" => OurSourceExtractorComponents::get(&filename).map(|f| {
            std::str::from_utf8(&f.data)
                .unwrap_or("Binary file")
                .to_string()
        }),
        "src/extractor/model" => OurSourceExtractorModel::get(&filename).map(|f| {
            std::str::from_utf8(&f.data)
                .unwrap_or("Binary file")
                .to_string()
        }),
        "src/extractor/system" => OurSourceExtractorSystem::get(&filename).map(|f| {
            std::str::from_utf8(&f.data)
                .unwrap_or("Binary file")
                .to_string()
        }),
        "src/model" => OurSourceModel::get(&filename).map(|f| {
            std::str::from_utf8(&f.data)
                .unwrap_or("Binary file")
                .to_string()
        }),
        "src/model/git" => OurSourceModelGit::get(&filename).map(|f| {
            std::str::from_utf8(&f.data)
                .unwrap_or("Binary file")
                .to_string()
        }),
        "src/model/lean" => OurSourceModelLean::get(&filename).map(|f| {
            std::str::from_utf8(&f.data)
                .unwrap_or("Binary file")
                .to_string()
        }),
        "src/model/lean/types" => OurSourceModelLeanTypes::get(&filename).map(|f| {
            std::str::from_utf8(&f.data)
                .unwrap_or("Binary file")
                .to_string()
        }),
        "src/model/math" => OurSourceModeMath::get(&filename).map(|f| {
            std::str::from_utf8(&f.data)
                .unwrap_or("Binary file")
                .to_string()
        }),
        "src/playground" => OurSourcePlayground::get(&filename).map(|f| {
            std::str::from_utf8(&f.data)
                .unwrap_or("Binary file")
                .to_string()
        }),
        "src/state" => OurSourceState::get(&filename).map(|f| {
            std::str::from_utf8(&f.data)
                .unwrap_or("Binary file")
                .to_string()
        }),
        "src/views" => OurSourceView::get(&filename).map(|f| {
            std::str::from_utf8(&f.data)
                .unwrap_or("Binary file")
                .to_string()
        }),
        "src/views/component_memes" => OurSourceViewComponent::get(&filename).map(|f| {
            std::str::from_utf8(&f.data)
                .unwrap_or("Binary file")
                .to_string()
        }),
        "src/views/crypto_frontend" => OurSourceViewCrypto::get(&filename).map(|f| {
            std::str::from_utf8(&f.data)
                .unwrap_or("Binary file")
                .to_string()
        }),
        "src/views/extras_views" => OurSourceViewextra::get(&filename).map(|f| {
            std::str::from_utf8(&f.data)
                .unwrap_or("Binary file")
                .to_string()
        }),
        "src/views/wikidata_memes" => OurSourceViewWikwidata::get(&filename).map(|f| {
            std::str::from_utf8(&f.data)
                .unwrap_or("Binary file")
                .to_string()
        }),
        "src/views/workflow_memes" => OurSourceViewWorkflow::get(&filename).map(|f| {
            std::str::from_utf8(&f.data)
                .unwrap_or("Binary file")
                .to_string()
        }),
        _ => None,
    });

    rsx! {
        div { class: "source-browser",
            div { class: "browser-header",
                h2 { "Source Code Browser" }
            }
            div { class: "browser-content",
                ModuleSidebar { modules: modules.clone(), selected_module: selected_module.clone(), set_selected_module: selected_module.clone(), set_selected_file: selected_file.clone(), files: files.clone(), selected_file: selected_file.clone() }
                div { class: "content-area",
                    FileViewer { file_content: file_content.clone(), selected_file: selected_file.clone() }
                }
            }
        }
        source_browser_style::SourceBrowserStyle {}
    }
}

#[component]
fn ModuleSidebar(
    modules: Vec<(&'static str, &'static str)>,
    selected_module: Signal<String>,
    set_selected_module: Signal<String>,
    set_selected_file: Signal<Option<String>>,
    files: Vec<std::borrow::Cow<'static, str>>,
    selected_file: Signal<Option<String>>,
) -> Element {
    rsx! {
        div { class: "sidebar",
            h3 { "Modules" }
            ul { class: "module-list",
                for (path, name) in modules {
                    li {
                        class: if selected_module() == path { "selected" } else { "" },
                        onclick: move |_| {
                            set_selected_module.set(path.to_string());
                            set_selected_file.set(None);
                        },
                        "{name}"
                    }
                }
            }
            if !files.is_empty() {
                FileList { files: files.clone(), selected_file: selected_file.clone(), set_selected_file: set_selected_file.clone() }
            }
        }
    }
}

#[component]
fn FileList(
    files: Vec<std::borrow::Cow<'static, str>>,
    selected_file: Signal<Option<String>>,
    set_selected_file: Signal<Option<String>>,
) -> Element {
    rsx! {
        div {
            h3 { "Files" }
            ul { class: "file-list",
                for file in files {
                    li {
                        class: if selected_file().as_ref() == Some(&file.to_string()) { "selected" } else { "" },
                        onclick: move |_| set_selected_file.set(Some(file.to_string())),
                        "{file}"
                    }
                }
            }
        }
    }
}

#[component]
fn FileViewer(file_content: Option<String>, selected_file: Signal<Option<String>>) -> Element {
    rsx! {
        if let Some(content) = file_content {
            div { class: "file-viewer",
                h3 { "File: {selected_file().unwrap_or_default()}" }
                pre { class: "code-content",
                    code { "{content}" }
                }
            }
        } else {
            div { class: "placeholder",
                p { "Select a file to view its content" }
            }
        }
    }
}
