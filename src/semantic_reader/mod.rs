use dioxus::prelude::*;
use sensiblaw_reader_model::{
    CoordinateCoverage, PropositionPayment, ReaderDisposition, ReaderIntent,
};

pub mod mabo;

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
        ReaderDisposition::ExecuteBoundedWhy(cone) => ReaderViewState::Explanation {
            proposition_ref: cone.proposition_ref.as_str().to_owned(),
            source_revision_ref: cone.source.source_revision_ref().as_str().to_owned(),
            span_ref: cone.source.span_ref().as_str().to_owned(),
            support_refs: cone.support_refs,
            residual_refs: residual_refs([&cone.qualifier, &cone.defeater, &cone.comparator]),
            applicability_paid: cone.applicability_paid(),
            claim_truth_paid: cone.claim_truth_paid(),
        },
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
            class: "semantic-reader",
            h2 { "Semantic Reader" }
            p { "Evidence payment is evaluated by SLR; this component only dispatches reader intents." }
            div {
                style: "display: flex; gap: 0.5rem; flex-wrap: wrap;",
                button {
                    onclick: move |_| view.set(render_state(&source_payment, ReaderIntent::OpenSource)),
                    "Source"
                }
                button {
                    onclick: move |_| view.set(render_state(&why_payment, ReaderIntent::WhyClaim)),
                    "Why?"
                }
            }
            {render_view(rendered)}
        }
    }
}

fn render_view(view: ReaderViewState) -> Element {
    match view {
        ReaderViewState::Ready => rsx!(p { "Select Source or Why?" }),
        ReaderViewState::Source {
            proposition_ref,
            source_revision_ref,
            span_ref,
        } => rsx!(div {
            h3 { "Exact source" }
            p { "Proposition: {proposition_ref}" }
            p { "Revision: {source_revision_ref}" }
            p { "Span: {span_ref}" }
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
            h3 { "Bounded explanation" }
            p { "Proposition: {proposition_ref}" }
            p { "Source revision: {source_revision_ref}" }
            p { "Source span: {span_ref}" }
            p { "Support: {support_refs.join(", ")}" }
            if !residual_refs.is_empty() {
                p { "Open coordinates: {residual_refs.join(", ")}" }
            }
            p { "Applicability paid: {applicability_paid}" }
            p { "Claim truth paid: {claim_truth_paid}" }
        }),
        ReaderViewState::Deferred { residual_refs } => rsx!(div {
            h3 { "More evidence is needed" }
            p { "Open coordinates: {residual_refs.join(", ")}" }
        }),
        ReaderViewState::Rejected { reason } => rsx!(div {
            h3 { "Reader action rejected" }
            p { "{reason}" }
        }),
    }
}
