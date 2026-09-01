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

- Repository quality execution is pinned to `ubuntu-24.04` and verifies Rust formatting, Clippy, tests, rustdoc, and fail-closed coverage on the exact source revision.
- `PublicationRequest` now carries caller intent only (`content_release_id` plus target); approval, source identity, locale, contract/version/standard, and blocking features come from authority ports instead of caller assertions.
- `PublicationOutcome` and `PublicationMetadata` remain externally read-only, and metadata now preserves release-approval and target-validation evidence identities.
- Target compatibility evidence for another release identity or source hash now fails closed with typed mismatch errors, preventing stale cached validation from authorizing a different immutable release.
- Empty or whitespace-only authority fields remain distinct from malformed SHA-256 identities through typed errors.
- Coverage enforcement is production-file scoped: every repository `src/` file must have 100% line and branch coverage, so test-only code cannot offset uncovered production paths.
