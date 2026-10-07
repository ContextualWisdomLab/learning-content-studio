# Changelog

## Unreleased

### Added

- Initial LCMS authoring authority boundary.
- Deterministic publication contract and publisher targets.
- Accessibility and learning-content standards traceability.
- Repository development rules.
- First executable Rust Publication Admission kernel.
- Fail-closed native-web vs cmi5 Quartz publisher-contract ownership validation.
- Deterministic machine-readable incompatibility ordering and duplicate rejection tests.
- Product/technical commercialization gap baseline and DDD context map.
- Explicit `ReleaseAuthorityPort` and `TargetCompatibilityPort` trust boundaries with authority evidence identities and regression coverage.
- `CompatibilityReleaseIdentity` binds target-validation evidence to the exact immutable `content_release_id` and `source_hash` it evaluated.

### Changed

- Repository quality execution is pinned to `ubuntu-24.04`, explicitly checks out and verifies the pull-request head SHA, and verifies Rust formatting, Clippy, tests, rustdoc, and fail-closed coverage on that exact source revision.
- `PublicationRequest` now carries caller intent only (`content_release_id` plus target); approval, source identity, locale, contract/version/standard, and blocking features come from authority ports instead of caller assertions.
- `PublicationOutcome` and `PublicationMetadata` remain externally read-only, and metadata now preserves release-approval and target-validation evidence identities.
- `BlockingFeature` identity fields are now encapsulated behind read-only accessors, preserving a validation boundary for future invariant changes.
- Target compatibility evidence for another release identity or source hash now fails closed with typed mismatch errors, preventing stale cached validation from authorizing a different immutable release.
- Empty or whitespace-only authority fields remain distinct from malformed SHA-256 identities through typed errors.
- Coverage enforcement inventories every repository `src/**/*.rs` file, requires an LLVM record for each, and then requires 100% statement/region, line, and branch coverage per file, so unlinked production code and test-only code cannot create false GREEN evidence.
- Coverage output no longer uses `cargo-llvm-cov`'s self-cleaned `target/llvm-cov/` work directory.
- Publication admission now consumes the request command fields it accepts by value, satisfying the denied Clippy ownership lint without changing the public call contract.
- Cargo dependency resolution is committed, while generated `target/` artifacts are excluded from repository state.
- Exact-head checkout assertion and repository-wide tracked-whitespace validation.
- ADR 0001 remains Proposed until ordinary protected integration.
