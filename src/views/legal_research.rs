use dioxus::prelude::*;
use sensiblaw_reader_model::{
    mabo_five_stage_registry, AdaptiveExplanationCone, ReaderIntent, ReaderPropositionSpec,
    ReaderWorldProjection, SourceCoordinate,
};

fn intent_label(intent: ReaderIntent) -> &'static str {
    match intent {
        ReaderIntent::Explain => "Explain",
        ReaderIntent::WhyClaim => "Why claim?",
        ReaderIntent::OpenSource => "Open source",
        ReaderIntent::ExpandProofCone => "Expand proof cone",
        ReaderIntent::Back => "Back",
    }
}

#[component]
pub fn LegalResearchWorkbench() -> Element {
    // Interaction state contains UI intent only.  It does not contain or mutate
    // evidence-payment, applicability, claim-truth, or authority state.
    let mut last_intent = use_signal(|| None::<ReaderIntent>);
    let registry = mabo_five_stage_registry();

    rsx! {
        main { class: "w-full max-w-7xl mx-auto px-6 py-8 space-y-8",
            header { class: "space-y-2",
                h1 { class: "text-3xl font-bold", "SensibLaw research workbench" }
                p { class: "text-sm opacity-75",
                    "Read-only projection shell. Dioxus emits ReaderIntent; semantic payment remains owned by SLR."
                }
            }

            section { class: "grid md:grid-cols-5 gap-2",
                for intent in [
                    ReaderIntent::Explain,
                    ReaderIntent::WhyClaim,
                    ReaderIntent::OpenSource,
                    ReaderIntent::ExpandProofCone,
                    ReaderIntent::Back,
                ] {
                    button {
                        class: "rounded border px-3 py-2 text-left hover:bg-slate-100 dark:hover:bg-slate-800",
                        onclick: move |_| last_intent.set(Some(intent)),
                        "{intent_label(intent)}"
                    }
                }
            }

            if let Some(intent) = last_intent() {
                aside { class: "rounded border p-3 text-sm",
                    strong { "Typed UI intent: " }
                    code { "{intent:?}" }
                    p { class: "opacity-70 mt-1",
                        "No semantic state changed. A backend/typed adapter must resolve this intent."
                    }
                }
            }

            TimelineRenderer { entries: registry }
        }
    }
}

#[component]
pub fn TimelineRenderer(entries: Vec<ReaderPropositionSpec>) -> Element {
    rsx! {
        section { class: "space-y-3",
            h2 { class: "text-xl font-semibold", "Mabo reading timeline" }
            ol { class: "space-y-3 border-l pl-5",
                for entry in entries {
                    li { class: "relative rounded border p-3",
                        div { class: "font-medium", "{entry.label}" }
                        code { class: "text-xs opacity-70", "{entry.proposition_ref}" }
                        match entry.source {
                            SourceCoordinate::Paid { source_revision_ref, span_ref } => rsx! {
                                div { class: "mt-2 text-xs",
                                    span { class: "font-semibold", "Paid exact source" }
                                    div { code { "{source_revision_ref}" } }
                                    div { code { "{span_ref}" } }
                                }
                            },
                            SourceCoordinate::Residual { residual_ref } => rsx! {
                                div { class: "mt-2 text-xs",
                                    span { class: "font-semibold", "Explicit residual" }
                                    div { code { "{residual_ref}" } }
                                }
                            },
                        }
                    }
                }
            }
        }
    }
}

/// Deterministic, renderer-only placement for a read-only world projection.
/// The position is based only on stable node order and has no legal meaning.
fn graph_position(index: usize, count: usize) -> (f64, f64) {
    if count == 0 {
        return (50.0, 50.0);
    }
    let columns = ((count as f64).sqrt().ceil() as usize).max(1);
    let row = index / columns;
    let column = index % columns;
    (
        10.0 + (column as f64 * 80.0 / columns as f64),
        12.0 + (row as f64 * 18.0),
    )
}

#[component]
pub fn WorldGraphRenderer(projection: ReaderWorldProjection) -> Element {
    let count = projection.nodes.len();
    rsx! {
        section { class: "space-y-2",
            h2 { class: "text-xl font-semibold", "World graph" }
            p { class: "text-xs opacity-70",
                "Layout position is presentation-only; rendered edges create no semantic authority."
            }
            div { class: "relative min-h-[28rem] rounded border overflow-auto",
                for (index, node) in projection.nodes.iter().enumerate() {
                    {
                        let (x, y) = graph_position(index, count);
                        rsx! {
                            button {
                                class: "absolute rounded border bg-white dark:bg-slate-900 px-2 py-1 text-xs max-w-48 truncate",
                                style: "left:{x}%; top:{y}%; transform:translate(-50%,-50%);",
                                title: "{node.node_ref}",
                                "{node.node_ref}"
                            }
                        }
                    }
                }
                div { class: "absolute bottom-2 left-2 right-2 max-h-36 overflow-auto text-xs space-y-1",
                    for edge in projection.edges.iter() {
                        div {
                            code { "{edge.from_ref}" }
                            span { " —{edge.kind:?}→ " }
                            code { "{edge.to_ref}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ProofTopologyRenderer(cone: AdaptiveExplanationCone) -> Element {
    rsx! {
        section { class: "space-y-2",
            h2 { class: "text-xl font-semibold", "Proof topology" }
            div { class: "text-xs opacity-70",
                "Focus: " code { "{cone.focus_ref}" }
                " · omitted by budget: {cone.omitted_count}"
            }
            ul { class: "grid md:grid-cols-2 gap-2",
                for node in cone.nodes {
                    li { class: "rounded border p-2 text-xs",
                        div { class: "font-medium", "{node.node_ref}" }
                        div { "kind={node.kind:?} · distance={node.distance} · score={node.elucidatory_score}" }
                        if node.mandatory {
                            strong { "mandatory paid/residual coordinate" }
                        }
                        if let Some(residual) = node.residual_ref {
                            div { "residual: " code { "{residual}" } }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn UI_intents_are_exact_reader_intents() {
        assert_eq!(intent_label(ReaderIntent::WhyClaim), "Why claim?");
        assert_eq!(intent_label(ReaderIntent::OpenSource), "Open source");
    }

    #[test]
    fn graph_layout_is_deterministic_and_semantically_empty() {
        assert_eq!(graph_position(0, 4), graph_position(0, 4));
        assert_ne!(graph_position(0, 4), graph_position(1, 4));
    }
}
