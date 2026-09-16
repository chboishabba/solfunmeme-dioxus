//! Dioxus Semantic Reader component for Mabo radical title / native title.
//!
//! Emits only `ReaderIntent`. Never constructs payment state or invents authority.

use dioxus::prelude::*;
use sensiblaw_reader_model::{
    CoordinateCoverage, PropositionPayment, ReaderDisposition, ReaderIntent, SemanticRef,
    SourcePayment, SpanRef,
};

use super::render_state;

pub const MABO_PROPOSITION_REF: &str = "mabo:proposition:radical-title-native-title";
pub const MABO_SOURCE_REVISION_REF: &str = "source-revision:mabo-hca23";
pub const MABO_BRENNAN_SPAN_REF: &str =
    "span:mabo:brennan:radical-title:no-automatic-beneficial-ownership";

/// Construct the canonical verified Mabo proposition payment projection.
#[must_use]
pub fn canonical_mabo_payment() -> PropositionPayment {
    let source = SourcePayment::paid(
        SemanticRef::new(MABO_PROPOSITION_REF),
        MABO_SOURCE_REVISION_REF,
        SpanRef::new(MABO_BRENNAN_SPAN_REF),
    );
    PropositionPayment::bounded(
        source,
        vec!["observation:mabo:radical-title:brennan-p39".into()],
        CoordinateCoverage::Residualised("reader-residual:qualifier".into()),
        CoordinateCoverage::Residualised("reader-residual:defeater".into()),
        CoordinateCoverage::Residualised("reader-residual:comparator".into()),
    )
}

#[derive(Clone, PartialEq, Props)]
pub struct MaboSemanticReaderProps {
    #[props(default = canonical_mabo_payment())]
    pub payment: PropositionPayment,
}

#[component]
pub fn MaboSemanticReader(props: MaboSemanticReaderProps) -> Element {
    let mut active_intent = use_signal(|| ReaderIntent::Explain);
    let view_state = render_state(&props.payment, *active_intent.read());

    rsx! {
        div { class: "p-6 max-w-4xl mx-auto bg-white dark:bg-gray-900 rounded-xl shadow-md space-y-4 border border-gray-200 dark:border-gray-800",
            div { class: "border-b pb-3 border-gray-200 dark:border-gray-700",
                h2 { class: "text-2xl font-bold text-gray-900 dark:text-white",
                    "Mabo v Queensland (No 2) — Semantic Reader"
                }
                p { class: "text-sm text-gray-500 dark:text-gray-400 font-mono mt-1",
                    "{MABO_PROPOSITION_REF}"
                }
            }

            // Legal thesis summary
            div { class: "bg-gray-50 dark:bg-gray-800 p-4 rounded-lg",
                p { class: "text-base text-gray-800 dark:text-gray-200 leading-relaxed",
                    "Radical title acquired upon the Crown's assertion of sovereignty does not confer automatic beneficial ownership of land occupied by Indigenous inhabitants."
                }
                p { class: "text-xs text-gray-500 dark:text-gray-400 mt-2",
                    "Brennan J (Mason CJ and McHugh J concurring), (1992) 175 CLR 1 at 39."
                }
            }

            // Action toolbar: Source and Why
            div { class: "flex items-center space-x-3 pt-2",
                button {
                    class: if view_state.can_execute_source {
                        "px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white text-sm font-semibold rounded shadow transition"
                    } else {
                        "px-4 py-2 bg-gray-400 text-gray-200 text-sm font-semibold rounded cursor-not-allowed"
                    },
                    disabled: !view_state.can_execute_source,
                    onclick: move |_| active_intent.set(ReaderIntent::OpenSource),
                    "Open Exact Source"
                }

                button {
                    class: if view_state.can_execute_why {
                        "px-4 py-2 bg-green-600 hover:bg-green-700 text-white text-sm font-semibold rounded shadow transition"
                    } else {
                        "px-4 py-2 bg-yellow-600 hover:bg-yellow-700 text-white text-sm font-semibold rounded shadow transition"
                    },
                    onclick: move |_| active_intent.set(ReaderIntent::WhyClaim),
                    if view_state.can_execute_why { "Why? (Bounded Explanation)" } else { "Why? (Deferred)" }
                }

                button {
                    class: "px-4 py-2 bg-gray-600 hover:bg-gray-700 text-white text-sm font-semibold rounded shadow transition",
                    onclick: move |_| active_intent.set(ReaderIntent::Explain),
                    "Reset View"
                }
            }

            // Active disposition disclosure
            div { class: "mt-4 p-4 rounded border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-950",
                h3 { class: "text-sm font-bold uppercase tracking-wider text-gray-700 dark:text-gray-300 mb-2",
                    "Reader Disposition"
                }
                match &view_state.disposition {
                    ReaderDisposition::ExecuteSource { proposition_ref, source_revision_ref, span_ref } => rsx! {
                        div { class: "space-y-1 text-sm font-mono text-blue-600 dark:text-blue-400",
                            p { "Disposition: ExecuteSource" }
                            p { "Proposition: {proposition_ref.as_str()}" }
                            p { "Source Revision: {source_revision_ref.as_str()}" }
                            p { "Exact Span: {span_ref.as_str()}" }
                        }
                    },
                    ReaderDisposition::ExecuteBoundedWhy(cone) => rsx! {
                        div { class: "space-y-2 text-sm text-green-700 dark:text-green-400",
                            p { class: "font-semibold", "Disposition: ExecuteBoundedWhy" }
                            p { class: "font-mono text-xs", "Proposition: {cone.proposition_ref.as_str()}" }
                            p { class: "text-xs", "Support refs: {cone.support_refs.len()} reviewed observation(s)" }
                            if !view_state.residuals.is_empty() {
                                div { class: "mt-2 pt-2 border-t border-gray-200 dark:border-gray-800",
                                    p { class: "font-semibold text-xs text-yellow-600 dark:text-yellow-400", "Retained Bounded Residuals:" }
                                    ul { class: "list-disc list-inside text-xs font-mono space-y-1 mt-1",
                                        for res in view_state.residuals.iter() {
                                            li { "{res}" }
                                        }
                                    }
                                }
                            }
                        }
                    },
                    ReaderDisposition::Defer(residuals) => rsx! {
                        div { class: "space-y-1 text-sm text-yellow-700 dark:text-yellow-400",
                            p { class: "font-semibold", "Disposition: Defer" }
                            p { "Outstanding residuals:" }
                            ul { class: "list-disc list-inside font-mono text-xs",
                                for res in residuals.iter() {
                                    li { "{res.as_str()}" }
                                }
                            }
                        }
                    },
                    ReaderDisposition::Reject { reason } => rsx! {
                        div { class: "text-sm text-gray-500 dark:text-gray-400 italic",
                            "{reason}"
                        }
                    },
                }
            }

            // Firewall footer
            div { class: "text-xs text-gray-400 dark:text-gray-500 pt-2 border-t border-gray-200 dark:border-gray-800 space-y-1",
                p { "Firewall Guarantee: Reachability, rendering, and button interaction cannot create semantic authority." }
                p { "applicability_paid: {view_state.applicability_paid} | claim_truth_paid: {view_state.claim_truth_paid}" }
            }
        }
    }
}
