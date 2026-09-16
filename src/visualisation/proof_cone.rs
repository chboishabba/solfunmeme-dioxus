//! Renderer-neutral Visual IR projection for ExplanationCones and Latent Worlds.
//!
//! Converts typed semantic reader models into layout/render graph structures
//! suitable for future wgpu pipelines, without creating or modifying semantic authority.

use sensiblaw_reader_model::{CoordinateCoverage, ExplanationCone};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VisualNodeKind {
    Proposition,
    SourceSpan,
    SupportObservation,
    ResidualCoordinate,
}

#[derive(Clone, Debug, PartialEq)]
pub struct VisualNode {
    pub id: String,
    pub label: String,
    pub kind: VisualNodeKind,
    pub is_residual: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct VisualEdge {
    pub source_id: String,
    pub target_id: String,
    pub relation: String,
    pub is_residual: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProofConeVisualIR {
    pub root_id: String,
    pub nodes: Vec<VisualNode>,
    pub edges: Vec<VisualEdge>,
    pub applicability_paid: bool,
    pub claim_truth_paid: bool,
}

impl ProofConeVisualIR {
    #[must_use]
    pub fn from_explanation_cone(cone: &ExplanationCone) -> Self {
        let root_id = cone.proposition_ref.as_str().to_string();
        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        // Root proposition node
        nodes.push(VisualNode {
            id: root_id.clone(),
            label: root_id.clone(),
            kind: VisualNodeKind::Proposition,
            is_residual: false,
        });

        // Exact source span node & edge
        let source_id = cone.source.span_ref().as_str().to_string();
        nodes.push(VisualNode {
            id: source_id.clone(),
            label: format!(
                "{}:{}",
                cone.source.source_revision_ref().as_str(),
                source_id
            ),
            kind: VisualNodeKind::SourceSpan,
            is_residual: false,
        });
        edges.push(VisualEdge {
            source_id: root_id.clone(),
            target_id: source_id,
            relation: "exact_source_span".into(),
            is_residual: false,
        });

        // Support observations
        for (idx, sup) in cone.support_refs.iter().enumerate() {
            let sup_id = format!("support:{}", idx);
            nodes.push(VisualNode {
                id: sup_id.clone(),
                label: sup.clone(),
                kind: VisualNodeKind::SupportObservation,
                is_residual: false,
            });
            edges.push(VisualEdge {
                source_id: root_id.clone(),
                target_id: sup_id,
                relation: "supports_proposition".into(),
                is_residual: false,
            });
        }

        // Qualifier
        Self::attach_coverage(&root_id, "qualifier", &cone.qualifier, &mut nodes, &mut edges);
        // Defeater
        Self::attach_coverage(&root_id, "defeater", &cone.defeater, &mut nodes, &mut edges);
        // Comparator
        Self::attach_coverage(
            &root_id,
            "comparator",
            &cone.comparator,
            &mut nodes,
            &mut edges,
        );

        Self {
            root_id,
            nodes,
            edges,
            applicability_paid: cone.applicability_paid(),
            claim_truth_paid: cone.claim_truth_paid(),
        }
    }

    fn attach_coverage(
        root_id: &str,
        role: &str,
        coverage: &CoordinateCoverage,
        nodes: &mut Vec<VisualNode>,
        edges: &mut Vec<VisualEdge>,
    ) {
        match coverage {
            CoordinateCoverage::Paid { evidence_refs } => {
                for (i, ev) in evidence_refs.iter().enumerate() {
                    let node_id = format!("{}:{}:paid", role, i);
                    nodes.push(VisualNode {
                        id: node_id.clone(),
                        label: ev.clone(),
                        kind: VisualNodeKind::SupportObservation,
                        is_residual: false,
                    });
                    edges.push(VisualEdge {
                        source_id: root_id.to_string(),
                        target_id: node_id,
                        relation: format!("covered_{role}"),
                        is_residual: false,
                    });
                }
            }
            CoordinateCoverage::Residualised(res) => {
                let node_id = format!("{}:residual", role);
                nodes.push(VisualNode {
                    id: node_id.clone(),
                    label: res.as_str().to_string(),
                    kind: VisualNodeKind::ResidualCoordinate,
                    is_residual: true,
                });
                edges.push(VisualEdge {
                    source_id: root_id.to_string(),
                    target_id: node_id,
                    relation: format!("residual_{role}"),
                    is_residual: true,
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sensiblaw_reader_model::{
        PropositionPayment, ReaderDisposition, ReaderIntent, SemanticRef, SourcePayment,
        SpanRef,
    };

    const MABO_PROP: &str = "mabo:proposition:radical-title-native-title";
    const MABO_REV: &str = "source-revision:mabo-hca23";
    const MABO_SPAN: &str = "span:mabo:brennan:radical-title:no-automatic-beneficial-ownership";

    #[test]
    fn test_proof_cone_visual_ir_deterministic_projection() {
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

        let disposition = payment.resolve(ReaderIntent::WhyClaim);
        let cone = match disposition {
            ReaderDisposition::ExecuteBoundedWhy(cone) => cone,
            other => panic!("expected ExecuteBoundedWhy, got {:?}", other),
        };

        let visual_ir = ProofConeVisualIR::from_explanation_cone(&cone);

        assert_eq!(visual_ir.root_id, MABO_PROP);
        assert!(!visual_ir.applicability_paid);
        assert!(!visual_ir.claim_truth_paid);

        // Nodes: root (1) + source span (1) + support (1) + 3 residuals (3) = 6 nodes
        assert_eq!(visual_ir.nodes.len(), 6);
        // Edges: root->source (1) + root->support (1) + root->qualifier (1) + root->defeater (1) + root->comparator (1) = 5 edges
        assert_eq!(visual_ir.edges.len(), 5);

        // Verify root node
        let root = visual_ir.nodes.iter().find(|n| n.id == MABO_PROP).unwrap();
        assert_eq!(root.kind, VisualNodeKind::Proposition);
        assert!(!root.is_residual);

        // Verify source span node
        let span_node = visual_ir.nodes.iter().find(|n| n.id == MABO_SPAN).unwrap();
        assert_eq!(span_node.kind, VisualNodeKind::SourceSpan);
        assert!(!span_node.is_residual);

        // Verify residual nodes
        let qualifier_node = visual_ir
            .nodes
            .iter()
            .find(|n| n.id == "qualifier:residual")
            .unwrap();
        assert_eq!(qualifier_node.kind, VisualNodeKind::ResidualCoordinate);
        assert!(qualifier_node.is_residual);
        assert_eq!(qualifier_node.label, "reader-residual:qualifier");

        // Verify residual edges
        let qualifier_edge = visual_ir
            .edges
            .iter()
            .find(|e| e.target_id == "qualifier:residual")
            .unwrap();
        assert!(qualifier_edge.is_residual);
        assert_eq!(qualifier_edge.relation, "residual_qualifier");
    }
}
