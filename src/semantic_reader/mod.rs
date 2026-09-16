//! Semantic Reader integration consuming typed SLR `sensiblaw-reader-model`.
//!
//! Dioxus owns only ordinary UI, interaction buttons, and progressive disclosure.
//! It never constructs semantic payment state or promotes truth/applicability.

pub mod mabo;

use sensiblaw_reader_model::{
    PropositionPayment, ReaderDisposition, ReaderIntent,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ReaderViewState {
    pub can_execute_source: bool,
    pub can_execute_why: bool,
    pub disposition: ReaderDisposition,
    pub residuals: Vec<String>,
    pub applicability_paid: bool,
    pub claim_truth_paid: bool,
}

/// Pure helper projecting a typed `PropositionPayment` and active `ReaderIntent`
/// into an immutable UI view state.
#[must_use]
pub fn render_state(payment: &PropositionPayment, intent: ReaderIntent) -> ReaderViewState {
    let disposition = payment.resolve(intent);
    let can_execute_source = matches!(
        payment.resolve(ReaderIntent::OpenSource),
        ReaderDisposition::ExecuteSource { .. }
    );
    let can_execute_why = matches!(
        payment.resolve(ReaderIntent::WhyClaim),
        ReaderDisposition::ExecuteBoundedWhy(_)
    );
    let residuals = match &disposition {
        ReaderDisposition::Defer(res) => res.iter().map(|r| r.as_str().to_string()).collect(),
        ReaderDisposition::ExecuteBoundedWhy(cone) => {
            let mut list = Vec::new();
            if let Some(r) = cone.qualifier.residual_ref() {
                list.push(r.as_str().to_string());
            }
            if let Some(r) = cone.defeater.residual_ref() {
                list.push(r.as_str().to_string());
            }
            if let Some(r) = cone.comparator.residual_ref() {
                list.push(r.as_str().to_string());
            }
            list
        }
        _ => Vec::new(),
    };

    ReaderViewState {
        can_execute_source,
        can_execute_why,
        disposition,
        residuals,
        applicability_paid: payment.applicability_paid(),
        claim_truth_paid: payment.claim_truth_paid(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sensiblaw_reader_model::{
        CoordinateCoverage, SemanticRef, SourcePayment, SpanRef,
    };

    const MABO_PROP: &str = "mabo:proposition:radical-title-native-title";
    const MABO_REV: &str = "source-revision:mabo-hca23";
    const MABO_SPAN: &str = "span:mabo:brennan:radical-title:no-automatic-beneficial-ownership";

    #[test]
    fn test_render_state_source_only_defers_why() {
        let source = SourcePayment::paid(
            SemanticRef::new(MABO_PROP),
            MABO_REV,
            SpanRef::new(MABO_SPAN),
        );
        let payment = PropositionPayment::source_only(source);

        let state_source = render_state(&payment, ReaderIntent::OpenSource);
        assert!(state_source.can_execute_source);
        assert!(!state_source.can_execute_why);
        assert!(matches!(
            state_source.disposition,
            ReaderDisposition::ExecuteSource { .. }
        ));
        assert!(!state_source.applicability_paid);
        assert!(!state_source.claim_truth_paid);

        let state_why = render_state(&payment, ReaderIntent::WhyClaim);
        assert!(state_why.can_execute_source);
        assert!(!state_why.can_execute_why);
        assert!(matches!(state_why.disposition, ReaderDisposition::Defer(_)));
        assert!(!state_why.residuals.is_empty());
    }

    #[test]
    fn test_render_state_bounded_executes_why() {
        let source = SourcePayment::paid(
            SemanticRef::new(MABO_PROP),
            MABO_REV,
            SpanRef::new(MABO_SPAN),
        );
        let payment = PropositionPayment::bounded(
            source,
            vec!["observation:mabo:radical-title:brennan-p39".into()],
            CoordinateCoverage::Residualised("reader-residual:qualifier".into()),
            CoordinateCoverage::Residualised("reader-residual:defeater".into()),
            CoordinateCoverage::Residualised("reader-residual:comparator".into()),
        );

        let state_why = render_state(&payment, ReaderIntent::WhyClaim);
        assert!(state_why.can_execute_source);
        assert!(state_why.can_execute_why);
        assert!(matches!(
            state_why.disposition,
            ReaderDisposition::ExecuteBoundedWhy(_)
        ));
        assert_eq!(state_why.residuals.len(), 3);
        assert!(!state_why.applicability_paid);
        assert!(!state_why.claim_truth_paid);
    }
}
