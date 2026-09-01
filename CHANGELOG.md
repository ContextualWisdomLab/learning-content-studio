# Changelog

## Unreleased

### Added

- Initial LCMS authoring authority boundary.
- Deterministic publication contract and publisher targets.
- Accessibility and learning-content standards traceability.
- Repository development rules.
- First executable Rust Publication Admission kernel for approved immutable releases.
- Fail-closed native-web vs cmi5 Quartz publisher-contract ownership validation.
- Deterministic machine-readable incompatibility ordering and duplicate rejection tests.
- Product/technical commercialization gap baseline and DDD context map.

### Changed

- Repository quality execution is pinned to `ubuntu-24.04` and now verifies Rust formatting, Clippy, tests, and rustdoc on the exact source revision.
- Trusted publication outcomes and their authority metadata are now externally read-only and can only be constructed by the validated admission path, preventing downstream callers from bypassing approval, hash, contract, or blocker checks.
