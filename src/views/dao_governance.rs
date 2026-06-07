use dioxus::prelude::*;

/// DAO Governance view — shows tier status, vote on bills, submit tx data
#[component]
pub fn DaoGovernance() -> Element {
    let mut vote_status = use_signal(|| "Connect wallet to vote".to_string());
    let mut bounty_count = use_signal(|| 2_532_473u64);

    rsx! {
        div { class: "dao-governance",
            h2 { "🏛️ SOLFUNMEME Federal DAO" }

            div { class: "dao-status",
                h3 { "Governance Status" }
                table {
                    tr { th { "Chamber" } th { "Seats" } th { "Threshold" } th { "Role" } }
                    tr { td { "Senate" } td { "100" } td { "51 majority" } td { "Upper house" } }
                    tr { td { "House" } td { "500" } td { "251 majority" } td { "Lower house" } }
                    tr { td { "Lobby" } td { "1000" } td { "Advisory" } td { "Non-binding" } }
                }
            }

            div { class: "dao-vote",
                h3 { "📋 Today's Bill" }
                p { "Daily ZK rollup — vote to include/exclude transactions" }
                div { class: "vote-buttons",
                    button {
                        class: "btn-yea",
                        onclick: move |_| vote_status.set("Vote: YEA submitted".into()),
                        "✅ YEA"
                    }
                    button {
                        class: "btn-nay",
                        onclick: move |_| vote_status.set("Vote: NAY submitted".into()),
                        "❌ NAY"
                    }
                    button {
                        class: "btn-abstain",
                        onclick: move |_| vote_status.set("Vote: ABSTAIN submitted".into()),
                        "⏭️ ABSTAIN"
                    }
                }
                p { class: "vote-status", "{vote_status}" }
            }

            div { class: "dao-bounty",
                h3 { "💰 Bounty Program" }
                p { "Earn 0.001 SOL per missing transaction submitted" }
                p { class: "bounty-remaining",
                    "Remaining: {bounty_count} transactions"
                }
                p { "Submit via RabbitMQ or POST to /solfunmeme/paste" }
            }

            div { class: "dao-proofs",
                h3 { "🔐 Lean4 Verified Properties" }
                ul {
                    li { "✓ Fibonacci tier boundaries strictly increasing" }
                    li { "✓ Bicameral: neither chamber can enact alone" }
                    li { "✓ Senate minority veto (51 block 500 reps)" }
                    li { "✓ NFT credential + ZK freshness required to vote" }
                    li { "✓ Sold tokens → vote rejected" }
                }
            }
        }
    }
}

crate::register_plugin!(
    "dao_governance",
    "Vote on daily ZK rollup bills",
    crate::plugin::PluginCategory::Dao,
    "🏛️",
    || rsx! { div{"plugin"} }
);
