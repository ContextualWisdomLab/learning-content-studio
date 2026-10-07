//! Regression contracts for exact-head and production coverage CI evidence.

const QUALITY_WORKFLOW: &str = include_str!("../.github/workflows/quality.yml");

#[test]
fn quality_checkout_is_bound_to_the_pull_request_head() {
    assert!(
        QUALITY_WORKFLOW.contains("ref: ${{ github.event.pull_request.head.sha || github.sha }}")
    );
    assert!(QUALITY_WORKFLOW.contains(
        "EXPECTED_CHECKOUT_SHA: ${{ github.event.pull_request.head.sha || github.sha }}"
    ));
    assert!(QUALITY_WORKFLOW.contains("git rev-parse HEAD"));
}

#[test]
fn tracked_files_are_checked_for_whitespace_regressions() {
    assert!(QUALITY_WORKFLOW.contains(
        "git diff --check \"$(git hash-object -t tree /dev/null)\" HEAD"
    ));
}

#[test]
fn production_coverage_requires_statement_regions() {
    assert!(QUALITY_WORKFLOW.contains("Path(\"target/coverage.json\")"));
    assert!(QUALITY_WORKFLOW.contains("summary.get(\"regions\")"));
    assert!(QUALITY_WORKFLOW.contains("region_covered != region_count"));
    assert!(QUALITY_WORKFLOW.contains("production statement/region coverage below 100%"));
}

#[test]
fn production_coverage_requires_every_source_file_in_report() {
    assert!(QUALITY_WORKFLOW.contains("(repository_root / \"src\").rglob(\"*.rs\")"));
    assert!(QUALITY_WORKFLOW.contains("production source files missing from coverage report"));
}

#[test]
fn rust_dependency_resolution_is_locked() {
    assert!(QUALITY_WORKFLOW.contains("cargo clippy --locked --all-targets"));
    assert!(QUALITY_WORKFLOW.contains("cargo test --locked --all-targets"));
    assert!(QUALITY_WORKFLOW.contains("cargo doc --locked --no-deps"));
    assert!(QUALITY_WORKFLOW.contains("\"llvm-cov\",\n                  \"--locked\""));
}
