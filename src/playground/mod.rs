pub mod app;
pub use app::MenuOption;

// Active playground modules
pub mod bert_test;
pub mod coverage_app;
pub mod doc_cleaner;
pub mod embedding;
pub mod markdown_processor;
pub mod mcp;
pub mod monster_meta_meme;
pub mod performance_charts;
pub mod rust_parser;
pub mod test_app;
pub mod test_components;
pub mod wikidata;

// Desktop-only (animations, heavy rendering)
#[cfg(not(target_arch = "wasm32"))]
pub mod orbits;
#[cfg(not(target_arch = "wasm32"))]
pub mod solfunmeme;
#[cfg(not(target_arch = "wasm32"))]
pub mod solfunnice;
#[cfg(not(target_arch = "wasm32"))]
pub mod test_emojis;
