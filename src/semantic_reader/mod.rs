//! Semantic Reader integration consuming typed SLR `sensiblaw-reader-model`.
//!
//! Dioxus owns only ordinary UI, interaction buttons, and progressive disclosure.
//! It never constructs semantic payment state or promotes truth/applicability.

use dioxus::prelude::*;
use sensiblaw_reader_model::{
    CoordinateCoverage, PropositionPayment, ReaderDisposition, ReaderIntent,
};

pub mod mabo;
pub use mabo::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReaderViewState {
    Ready,
    Source {
        proposition_ref: String,
        source_revision_ref: String,
        span_ref: String,
    },
    Explanation {
        proposition_ref: String,
        source_revision_ref: String,
        span_ref: String,
        support_refs: Vec<String>,
        residual_refs: Vec<String>,
        applicability_paid: bool,
        claim_truth_paid: bool,
    },
    Deferred {
        residual_refs: Vec<String>,
    },
    Rejected {
        reason: String,
    },
}

impl ReaderViewState {
    #[must_use]
    pub fn can_execute_source(&self) -> bool {
        matches!(self, Self::Source { .. })
    }

    #[must_use]
    pub fn can_execute_why(&self) -> bool {
        matches!(self, Self::Explanation { .. })
    }
}

fn residual_refs(coverages: [&CoordinateCoverage; 3]) -> Vec<String> {
    coverages
        .into_iter()
        .filter_map(CoordinateCoverage::residual_ref)
        .map(|residual| residual.as_str().to_owned())
        .collect()
}

/// Project an already-evaluated SLR payment into reader-only state.
///
/// This function does not inspect PostgreSQL, infer support, or mutate payment.
/// Dioxus supplies an intent and renders the resulting SLR disposition.
#[must_use]
pub fn render_state(payment: &PropositionPayment, intent: ReaderIntent) -> ReaderViewState {
    match payment.resolve(intent) {
        ReaderDisposition::ExecuteSource {
            proposition_ref,
            source_revision_ref,
            span_ref,
        } => ReaderViewState::Source {
            proposition_ref: proposition_ref.as_str().to_owned(),
            source_revision_ref: source_revision_ref.as_str().to_owned(),
            span_ref: span_ref.as_str().to_owned(),
        },
        ReaderDisposition::ExecuteBoundedWhy(cone) => {
            let proposition_ref = cone.proposition_ref.as_str().to_owned();
            let source_revision_ref = cone.source.source_revision_ref().as_str().to_owned();
            let span_ref = cone.source.span_ref().as_str().to_owned();
            let open_residual_refs =
                residual_refs([&cone.qualifier, &cone.defeater, &cone.comparator]);
            let applicability_paid = cone.applicability_paid();
            let claim_truth_paid = cone.claim_truth_paid();
            let support_refs = cone.support_refs;

            ReaderViewState::Explanation {
                proposition_ref,
                source_revision_ref,
                span_ref,
                support_refs,
                residual_refs: open_residual_refs,
                applicability_paid,
                claim_truth_paid,
            }
        }
        ReaderDisposition::Defer(residuals) => ReaderViewState::Deferred {
            residual_refs: residuals
                .into_iter()
                .map(|residual| residual.as_str().to_owned())
                .collect(),
        },
        ReaderDisposition::Reject { reason } => ReaderViewState::Rejected { reason },
    }
}

#[component]
pub fn SemanticReader(payment: PropositionPayment) -> Element {
    let mut view = use_signal(|| ReaderViewState::Ready);
    let source_payment = payment.clone();
    let why_payment = payment.clone();

    let rendered = view.read().clone();

    rsx! {
        section {
            class: "semantic-reader p-6 max-w-4xl mx-auto bg-white dark:bg-gray-900 rounded-xl shadow-md space-y-4 border border-gray-200 dark:border-gray-800",
            div { class: "border-b pb-3 border-gray-200 dark:border-gray-700",
                h2 { class: "text-2xl font-bold text-gray-900 dark:text-white", "Semantic Reader" }
                p { class: "text-sm text-gray-500 dark:text-gray-400 mt-1",
                    "Evidence payment is evaluated by SLR; this component only dispatches reader intents."
                }
            }
            div {
                style: "display: flex; gap: 0.5rem; flex-wrap: wrap;",
                button {
                    class: "px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white text-sm font-semibold rounded shadow transition",
                    onclick: move |_| view.set(render_state(&source_payment, ReaderIntent::OpenSource)),
                    "Open Exact Source"
                }
                button {
                    class: "px-4 py-2 bg-green-600 hover:bg-green-700 text-white text-sm font-semibold rounded shadow transition",
                    onclick: move |_| view.set(render_state(&why_payment, ReaderIntent::WhyClaim)),
                    "Why? (Bounded Explanation)"
                }
                button {
                    class: "px-4 py-2 bg-gray-600 hover:bg-gray-700 text-white text-sm font-semibold rounded shadow transition",
                    onclick: move |_| view.set(ReaderViewState::Ready),
                    "Reset View"
                }
            }
            {render_view(rendered)}
        }
    }
}

fn render_view(view: ReaderViewState) -> Element {
    match view {
        ReaderViewState::Ready => rsx!(div {
            class: "text-sm text-gray-500 dark:text-gray-400 italic mt-2",
            "Select 'Open Exact Source' or 'Why?' to dispatch a reader intent."
        }),
        ReaderViewState::Source {
            proposition_ref,
            source_revision_ref,
            span_ref,
        } => rsx!(div {
            class: "mt-4 p-4 rounded border border-blue-200 dark:border-blue-800 bg-blue-50 dark:bg-blue-950 font-mono text-sm space-y-1 text-blue-800 dark:text-blue-300",
            h3 { class: "font-bold text-base", "Exact Source Span" }
            p { "Proposition: {proposition_ref}" }
            p { "Source Revision: {source_revision_ref}" }
            p { "Span Ref: {span_ref}" }
        }),
        ReaderViewState::Explanation {
            proposition_ref,
            source_revision_ref,
            span_ref,
            support_refs,
            residual_refs,
            applicability_paid,
            claim_truth_paid,
        } => rsx!(div {
            class: "mt-4 p-4 rounded border border-green-200 dark:border-green-800 bg-green-50 dark:bg-green-950 text-sm space-y-2 text-green-900 dark:text-green-200",
            h3 { class: "font-bold text-base", "Bounded Explanation Cone" }
            p { class: "font-mono text-xs", "Proposition: {proposition_ref}" }
            p { class: "font-mono text-xs", "Source revision: {source_revision_ref}" }
            p { class: "font-mono text-xs", "Source span: {span_ref}" }
            p { class: "text-xs", "Support: {support_refs.join(\", \")}" }
            if !residual_refs.is_empty() {
                div { class: "pt-2 border-t border-green-200 dark:border-green-800",
                    p { class: "font-semibold text-xs text-yellow-700 dark:text-yellow-400", "Retained Residual Coordinates:" }
                    ul { class: "list-disc list-inside text-xs font-mono space-y-1 mt-1",
                        for res in residual_refs.iter() {
                            li { "{res}" }
                        }
                    }
                }
            }
            div { class: "pt-2 border-t border-green-200 dark:border-green-800 text-xs text-gray-500 dark:text-gray-400 space-y-1",
                p { "Firewall: Applicability paid: {applicability_paid} | Claim truth paid: {claim_truth_paid}" }
            }
        }),
        ReaderViewState::Deferred { residual_refs } => rsx!(div {
            class: "mt-4 p-4 rounded border border-yellow-200 dark:border-yellow-800 bg-yellow-50 dark:bg-yellow-950 text-sm space-y-1 text-yellow-900 dark:text-yellow-300",
            h3 { class: "font-bold text-base", "More evidence is needed (Deferred)" }
            p { "Open coordinates:" }
            ul { class: "list-disc list-inside font-mono text-xs",
                for res in residual_refs.iter() {
                    li { "{res}" }
                }
            }
        }),
        ReaderViewState::Rejected { reason } => rsx!(div {
            class: "mt-4 p-4 rounded border border-red-200 dark:border-red-800 bg-red-50 dark:bg-red-950 text-sm text-red-800 dark:text-red-300 italic",
            h3 { class: "font-bold text-base", "Reader action rejected" }
            p { "{reason}" }
        }),
    }
}
