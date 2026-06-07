use crate::model::erdfa::TOKEN_CA;
use dioxus::prelude::*;

/// P2P data sharing — connect wallet, share/receive tx data, stego storage
#[component]
pub fn P2pSharing() -> Element {
    let mut peer_count = use_signal(|| 0u32);
    let mut shared_count = use_signal(|| 0u32);
    let mut received_count = use_signal(|| 0u32);
    let mut stego_status = use_signal(|| "Not connected".to_string());

    rsx! {
        div { class: "p2p-view",
            h2 { "🌐 P2P Data Network" }

            div { class: "p2p-status",
                h3 { "Network Status" }
                table {
                    tr { td { "Peers" } td { "{peer_count}" } }
                    tr { td { "Shared" } td { "{shared_count} tx" } }
                    tr { td { "Received" } td { "{received_count} tx" } }
                    tr { td { "Stego Storage" } td { "{stego_status}" } }
                }
            }

            div { class: "p2p-wallet",
                h3 { "🔑 Wallet Connection" }
                p { "Connect your Solana wallet to:" }
                ul {
                    li { "Prove token holdings (ZK freshness)" }
                    li { "Sign NFT credentials for voting" }
                    li { "Submit tx data and earn bounties" }
                    li { "Store votes in stego images" }
                }
                p { class: "token-info", "Token: {TOKEN_CA}" }
            }

            div { class: "p2p-actions",
                h3 { "Actions" }
                button {
                    class: "btn-share",
                    onclick: move |_| {
                        shared_count += 1;
                        stego_status.set("Encoding to stego...".into());
                    },
                    "📤 Share TX Data"
                }
                button {
                    class: "btn-receive",
                    onclick: move |_| {
                        received_count += 1;
                    },
                    "📥 Receive TX Data"
                }
                button {
                    class: "btn-stego",
                    onclick: move |_| {
                        stego_status.set("Stego storage active".into());
                    },
                    "🖼️ Stego Encode"
                }
            }

            div { class: "p2p-channels",
                h3 { "📡 Channels" }
                table {
                    tr { th { "Channel" } th { "Type" } th { "Status" } }
                    tr { td { "Telegram" } td { "Public" } td { "Agent scraping" } }
                    tr { td { "Discord" } td { "Public" } td { "Agent scraping" } }
                    tr { td { "WireGuard" } td { "Private" } td { "Stego tunnel" } }
                    tr { td { "Direct API" } td { "HTTP" } td { "POST /solfunmeme/paste" } }
                    tr { td { "RabbitMQ" } td { "AMQP" } td { "solfunmeme exchange" } }
                    tr { td { "HuggingFace" } td { "Dataset" } td { "introspector/solfunmeme" } }
                }
            }
        }
    }
}

crate::register_plugin!(
    "p2p_sharing",
    "P2P data sharing + stego",
    crate::plugin::PluginCategory::Data,
    "🌐",
    || rsx! { div{"plugin"} }
);
