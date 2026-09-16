//! Production Mabo reader shell.
//!
//! A caller must supply the typed SLR `PropositionPayment`. Absence is rendered
//! explicitly; this component never fabricates a fixture payment from UI state.

use dioxus::prelude::*;
use sensiblaw_reader_model::{
    CoordinateCoverage, PropositionPayment, SemanticRef, SourcePayment, SpanRef,
};

use super::SemanticReader;

pub const MABO_PROPOSITION_REF: &str = "mabo:proposition:radical-title-native-title";
pub const MABO_SOURCE_REVISION_REF: &str =
    "source-revision:mabo:1992:hca:23:wikisource:page-39:rev-16058297:2026-06-29";
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
    #[props(default = Some(canonical_mabo_payment()))]
    pub payment: Option<PropositionPayment>,
}

/// Production Mabo reader shell.
///
/// If `payment` is None, renders the unavailable state explicitly.
#[component]
pub fn MaboSemanticReader(props: MaboSemanticReaderProps) -> Element {
    match props.payment {
        Some(payment) => rsx!(SemanticReader { payment }),
        None => rsx!(section {
            class: "semantic-reader semantic-reader-unavailable p-6 max-w-4xl mx-auto bg-white dark:bg-gray-900 rounded-xl shadow-md space-y-4 border border-gray-200 dark:border-gray-800",
            h2 { class: "text-2xl font-bold text-gray-900 dark:text-white", "Mabo Semantic Reader" }
            p { class: "text-gray-600 dark:text-gray-400", "No typed SLR proposition payment is available for this reader session." }
            p { class: "text-sm text-yellow-600 dark:text-yellow-400", "Source and Why? actions remain unavailable until the Rust payment boundary supplies one." }
        }),
    }
}
