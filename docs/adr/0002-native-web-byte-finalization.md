# ADR 0002: Native-web byte finalization is a separate trust boundary

- Status: Accepted
- Date: 2026-09-02

## Context

Publication Admission intentionally owns release approval, target/contract ownership and SHA-256 identity syntax but does not possess canonical immutable release bytes. A reviewer correctly identified that admission therefore cannot prove that the admitted digest matches the actual release bytes. Treating a compatible admission as byte-integrity evidence would allow a downstream renderer to attach a syntactically valid but unrelated digest to emitted artifacts.

The publishing contract also requires exact `source_hash`, `artifact_hash`, `build_manifest_hash` and validation receipt identities. These values must not be trusted merely because a caller supplied strings with the right shape. At the same time, this repository does not yet have the shared released xAPI 2.0 contract needed to define a complete native-web renderer without duplicating authority from `ContextualWisdomLab/learning-interoperability-contracts`.

## Decision

Create a distinct native-web finalization domain service after rendering and before durable publication/storage.

`finalize_native_web_publication` accepts only a trusted compatible `PublicationOutcome` for `native_cwl_xapi_2_0/v1`, the exact canonical immutable release bytes, already-emitted artifact bytes, exact build-manifest bytes, and validation-receipt identities. It recomputes SHA-256 for the release/artifact/manifest byte sets, fails closed when release bytes disagree with admitted authority, canonicalizes validation-receipt identity order, rejects empty/duplicate evidence identities, and returns an opaque `NativeWebPublicationReceipt`.

SHA-256 uses pinned RustCrypto `sha2` 0.11.0 rather than handwritten cryptographic code. FIPS 180-4 is the current published SHA-256 algorithm authority; this is an algorithm-contract decision, not a claim that the crate or product is a FIPS-validated cryptographic module.

The finalizer does not render learning content, validate an xAPI profile, persist receipts, upload artifacts, or certify interoperability. Those responsibilities remain separate boundaries.

## Consequences

Positive:

- admission remains narrow and cannot be misread as byte equality;
- source/artifact/build-manifest hashes become computed evidence rather than caller assertions;
- native and cmi5 contracts remain isolated;
- repeated byte inputs and validation receipt identities produce deterministic receipt evidence;
- a future renderer can be replaced independently while retaining the same finalization invariant.

Costs/constraints:

- a new production dependency requires protected dependency/security evidence before integration;
- callers must retain or stream exact canonical release and emitted bytes through the finalization boundary;
- the repository still does not have a buyer-facing native renderer; that remains blocked on a released shared native/xAPI 2.0 contract and explicit transformation specification.

## Verification

Regression/edge-case tests were committed first at `2534b18c3422e6f353cc74c3443f8834d4f58cd2`. They require exact known SHA-256 outputs, repeatable receipt bytes independent of validation receipt input order, source mismatch rejection, cmi5 and incompatible admission rejection, case-equivalent digest comparison, empty artifact/build-manifest rejection, and empty/duplicate validation receipt rejection.

The exact-head quality workflow must pass formatting, Clippy, all-target tests, 100% line/branch coverage and rustdoc. Protected Dependency Review/SAST/security and independent review remain merge gates; unavailable dependency evidence fails closed.
