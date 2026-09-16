use sensiblaw_reader_model::{
    CoordinateCoverage, PropositionPayment, ReaderIntent, SemanticRef, SourcePayment, SpanRef,
};
use solfunmeme_dioxus::semantic_reader::{render_state, ReaderViewState};

const PROPOSITION: &str = "mabo:proposition:radical-title-native-title";
const SOURCE_REVISION: &str = "source-revision:mabo:1992:hca:23:wikisource:page-39:rev-16058297:2026-06-29";
const SPAN: &str = "span:mabo:brennan:radical-title:no-automatic-beneficial-ownership";

fn source_payment() -> SourcePayment {
    SourcePayment::paid(
        SemanticRef::new(PROPOSITION),
        SOURCE_REVISION,
        SpanRef::new(SPAN),
    )
}

fn bounded_payment() -> PropositionPayment {
    PropositionPayment::bounded(
        source_payment(),
        vec!["observation:mabo:radical-title:support".into()],
        CoordinateCoverage::Residualised("reader-residual:qualifier".into()),
        CoordinateCoverage::Residualised("reader-residual:defeater".into()),
        CoordinateCoverage::Residualised("reader-residual:comparator".into()),
    )
}

#[test]
fn bounded_why_is_projected_without_promoting_applicability_or_truth() {
    let view = render_state(&bounded_payment(), ReaderIntent::WhyClaim);

    match view {
        ReaderViewState::Explanation {
            proposition_ref,
            source_revision_ref,
            span_ref,
            support_refs,
            residual_refs,
            applicability_paid,
            claim_truth_paid,
        } => {
            assert_eq!(proposition_ref, PROPOSITION);
            assert_eq!(source_revision_ref, SOURCE_REVISION);
            assert_eq!(span_ref, SPAN);
            assert_eq!(support_refs, vec!["observation:mabo:radical-title:support"]);
            assert_eq!(
                residual_refs,
                vec![
                    "reader-residual:qualifier",
                    "reader-residual:defeater",
                    "reader-residual:comparator",
                ]
            );
            assert!(!applicability_paid);
            assert!(!claim_truth_paid);
        }
        other => panic!("expected bounded explanation, got {other:?}"),
    }
}

#[test]
fn source_only_payment_executes_source_but_defers_why() {
    let payment = PropositionPayment::source_only(source_payment());

    assert!(matches!(
        render_state(&payment, ReaderIntent::OpenSource),
        ReaderViewState::Source {
            proposition_ref,
            source_revision_ref,
            span_ref,
        } if proposition_ref == PROPOSITION
            && source_revision_ref == SOURCE_REVISION
            && span_ref == SPAN
    ));

    assert!(matches!(
        render_state(&payment, ReaderIntent::WhyClaim),
        ReaderViewState::Deferred { residual_refs }
            if residual_refs.iter().any(|residual| residual == "reader-residual:proposition-support")
    ));
}
