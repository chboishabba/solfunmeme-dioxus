# DA51 × SLR proof graph consumer boundary

Date: 2026-10-17

`solfunmeme-dioxus` is an operator/query cockpit for the cross-repo proof/evidence graph. It is not the owner of PNF, proof payment, DA51 semantics, artifact identity, or theorem certification.

Read-only composition:

```text
SLR PNF / residual / proof obligation
  + dashi-repo route/history
  + exact/equivalent/similar repository matches
  + DA51 identity/evidence
  + formal checker evidence
  + zkperf/eBPF runtime evidence
  + ITIR artifact/shard/object refs
  -> one operator view
```

UI identity must reuse producer-owned refs. A selected card/node must not mint a replacement theorem/proposition/obligation identity.

Hard boundaries:

```text
Dioxus selection != review/admission
Dioxus route display != proof
Dioxus DA51 display != theorem authority
Dioxus proof badge != SLR payment
Dioxus artifact/CID link != semantic identity
Dioxus runtime trace display != security conclusion
```

The canonical formal-payment join is exact `demand_ref + obligation_ref + statement_digest`. The UI may display that join and its evidence history; it may not recompute or loosen it.

Browser/WASM should continue using backend/HTTP/WebSocket mediation for MCP/repository services. Native/fullstack clients may call the read-oriented MCP surfaces directly. Relevant tools include `proof_route_for_obligation`, `formal_evidence_for_obligation`, `da51_resolve`, `da51_evidence`, and `da51_frontier`.
