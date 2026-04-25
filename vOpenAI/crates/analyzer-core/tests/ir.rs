use analyzer_core::ir::model::{Evidence, Finding, Severity, SourceSpan};

#[test]
fn finding_keeps_evidence_spans() {
    let span = SourceSpan::new("src/app.ts", 3, 5, 7, 1);
    let evidence = Evidence::new("signature changed").with_span(span.clone());

    let finding = Finding::new(
        "api.break",
        Severity::High,
        0.92,
        "Breaking API change",
        "The method signature changed.",
    )
    .with_evidence(evidence);

    assert_eq!(finding.code, "api.break");
    assert_eq!(finding.evidence.len(), 1);
    assert_eq!(finding.evidence[0].spans[0], span);
}
