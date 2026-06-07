//! Plugin trait for loadable playground modules.
//!
//! Each playground module registers as a Plugin with:
//! - name, description, version
//! - a render function that returns a Dioxus Element
//! - optional zkperf witness constraints
//!
//! Follows the zos-server PluginRegistry + VerifiedPluginLoader pattern.
//! Each plugin proves it does exactly what it says via zkperf witnesses.

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

/// Plugin metadata — what this module claims to do
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PluginMeta {
    pub name: &'static str,
    pub description: &'static str,
    pub version: &'static str,
    pub author: &'static str,
    pub category: PluginCategory,
    /// zkperf constraint: max render time in ms
    pub max_render_ms: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum PluginCategory {
    Dao,      // governance, voting, tiers
    Data,     // pastebin, tx submission, bounties
    Crypto,   // wallet, encryption, signing
    Analysis, // code parsing, coverage, embeddings
    Viz,      // charts, orbits, animations
    Meta,     // memes, ontology, MCP
    Test,     // test harnesses
}

/// Plugin registration — collected at startup via inventory
pub struct PluginRegistration {
    pub meta: PluginMeta,
    pub render: fn() -> Element,
    pub menu_label: &'static str,
    pub icon: &'static str,
}

// Global plugin registry
inventory::collect!(PluginRegistration);

/// Get all registered plugins
pub fn all_plugins() -> Vec<&'static PluginRegistration> {
    inventory::iter::<PluginRegistration>.into_iter().collect()
}

/// Get plugins by category
pub fn plugins_by_category(cat: PluginCategory) -> Vec<&'static PluginRegistration> {
    all_plugins()
        .into_iter()
        .filter(|p| p.meta.category == cat)
        .collect()
}

/// Plugin browser component — shows all registered plugins with their claims
#[component]
pub fn PluginBrowser() -> Element {
    let plugins = all_plugins();
    let categories = vec![
        (PluginCategory::Dao, "🏛️ DAO"),
        (PluginCategory::Data, "📊 Data"),
        (PluginCategory::Crypto, "🔐 Crypto"),
        (PluginCategory::Analysis, "🔬 Analysis"),
        (PluginCategory::Viz, "📈 Visualization"),
        (PluginCategory::Meta, "🧬 Meta"),
        (PluginCategory::Test, "🧪 Test"),
    ];

    rsx! {
        div { class: "plugin-browser",
            h2 { "🔌 Plugin Registry ({plugins.len()} modules)" }
            for (cat, label) in categories.iter() {
                {
                    let cat_plugins = plugins.iter()
                        .filter(|p| &p.meta.category == cat)
                        .collect::<Vec<_>>();
                    if !cat_plugins.is_empty() {
                        rsx! {
                            h3 { "{label}" }
                            table {
                                tr { th { "" } th { "Plugin" } th { "Description" } th { "Version" } th { "Max ms" } }
                                for p in cat_plugins.iter() {
                                    tr {
                                        td { "{p.icon}" }
                                        td { strong { "{p.meta.name}" } }
                                        td { "{p.meta.description}" }
                                        td { "{p.meta.version}" }
                                        td { "{p.meta.max_render_ms.map(|m| m.to_string()).unwrap_or(\"∞\".into())}" }
                                    }
                                }
                            }
                        }
                    } else {
                        rsx! {}
                    }
                }
            }
        }
    }
}

/// Macro to register a playground module as a plugin
#[macro_export]
macro_rules! register_plugin {
    ($name:expr, $desc:expr, $cat:expr, $icon:expr, $render:expr) => {
        inventory::submit! {
            crate::plugin::PluginRegistration {
                meta: crate::plugin::PluginMeta {
                    name: $name,
                    description: $desc,
                    version: env!("CARGO_PKG_VERSION"),
                    author: "meta-introspector",
                    category: $cat,
                    max_render_ms: None,
                },
                render: $render,
                menu_label: $name,
                icon: $icon,
            }
        }
    };
}
