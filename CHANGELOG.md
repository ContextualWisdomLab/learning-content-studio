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
- Native-web byte-finalization boundary that recomputes the admitted release SHA-256 from exact immutable bytes and records exact artifact/build-manifest SHA-256 identities.
- Opaque deterministic `NativeWebPublicationReceipt` with canonical validation-receipt ordering and fail-closed source/contract/empty-byte/duplicate-evidence checks.

### Changed

- Repository quality execution is pinned to `ubuntu-24.04` and now verifies Rust formatting, Clippy, tests, and rustdoc on the exact source revision.
- Trusted publication outcomes and their authority metadata are now externally read-only and can only be constructed by the validated admission path, preventing downstream callers from bypassing approval, hash, contract, or blocker checks.
- Empty or whitespace-only `source_hash` is now reported as `EmptyRequiredField("source_hash")`; non-empty malformed SHA-256 identities remain `InvalidSourceHash`, preserving a stable machine-readable distinction between missing and malformed publisher input.
- SHA-256 byte identity is implemented with pinned RustCrypto `sha2` 0.11.0 rather than handwritten cryptography; protected dependency/security evidence remains mandatory before integration.
