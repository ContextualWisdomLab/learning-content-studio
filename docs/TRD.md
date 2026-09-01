# Technical requirements — Publication Admission and Native Byte Finalization

## Scope

This TRD covers the first executable Learning Content Studio publication trust chain. Publication Admission evaluates caller intent for one immutable release/target only after obtaining authoritative release/approval evidence and release-bound target-compatibility evidence. Native-web byte finalization then verifies the admitted source identity against exact immutable release bytes and records exact emitted artifact/build-manifest identities. Rendering, durable persistence, network services, deployment, and buyer-facing UI remain outside this slice.

## Runtime and API

The implementation is a Rust library crate in `src/lib.rs` with `unsafe_code = "forbid"` and `missing_docs = "deny"`. Publication Admission itself is deterministic application/domain logic. Native byte finalization uses pinned RustCrypto `sha2 = 0.11.0` for SHA-256; no handwritten cryptographic implementation is introduced.

Primary APIs:

```text
evaluate_publication(
    PublicationRequest,
    &dyn ReleaseAuthorityPort,
    &dyn TargetCompatibilityPort,
) -> Result<PublicationOutcome, AdmissionError>

finalize_native_web_publication(
    &PublicationOutcome,
    canonical_release_bytes,
    artifact_bytes,
    build_manifest_bytes,
    validation_receipt_ids,
) -> Result<NativeWebPublicationReceipt, NativeWebPublicationError>
```

`PublicationRequest` contains only caller intent: `content_release_id` and `PublisherTarget`. It has no caller-controlled approval boolean, source hash, locale, publisher contract/version/standard, or blocker vector.

`ReleaseAuthorityPort` owns immutable-release lookup. Its `ReleaseAuthorityEvidence` supplies release identity, SHA-256 source identity, locale, approval state, and stable approval-evidence identity. `TargetCompatibilityPort` owns target validation. Its `TargetCompatibilityEvidence` supplies a `CompatibilityReleaseIdentity` containing the exact immutable `content_release_id` and `source_hash` validated, plus target, contract/version/standard, stable validation-evidence identity, and zero or more blockers. Production adapters are security-sensitive ACLs and must derive evidence from authoritative stores/services, not request fields or synthetic/demo state.

## Publication Admission validation order

1. reject an empty requested release identity;
2. require release-authority evidence and reject a returned release identity that does not exactly match the request;
3. reject release authority evidence that is not approved;
4. validate authority-owned source hash, locale, and approval-evidence identity;
5. validate SHA-256 syntax while leaving byte equality to the byte-owning finalizer;
6. require target-compatibility evidence and reject evidence for another target;
7. reject target compatibility evidence whose `content_release_id` does not exactly match release-authority evidence;
8. reject target compatibility evidence whose `source_hash` does not exactly match release-authority evidence, so stale cached validation cannot authorize different immutable bytes;
9. validate target-owned contract/version/standard/validation-evidence identities;
10. reject a contract not owned by the requested target;
11. validate blocker identities, sort by `feature_code`, `source_component_reference`, `reason_code`, and reject exact duplicates;
12. mint an opaque compatible outcome only when the authority-supplied blocker set is empty; otherwise mint an incompatible outcome.

No mutable authoring state, caller approval claim, caller blocker omission, wall clock, environment locale, network-fetched content, random ID, or process ordering participates in admission.

## Native-web byte-finalization validation order

`finalize_native_web_publication` is a downstream trust boundary after rendering and before durable storage/delivery. It:

1. rejects any admission whose status is not `Compatible`;
2. rejects any trusted outcome whose contract is not `native_cwl_xapi_2_0/v1`;
3. recomputes SHA-256 over exact canonical immutable release bytes and compares the digest to the authority-backed admitted source identity; hexadecimal case is non-semantic;
4. rejects zero-byte emitted artifact and build-manifest payloads;
5. rejects empty validation-receipt identities, preserves each external identity exactly, sorts them lexically, and rejects exact duplicates;
6. computes `artifact_hash` and `build_manifest_hash` from the exact supplied byte sequences;
7. canonicalizes receipt `source_hash` to the lowercase SHA-256 identity recomputed from release bytes while preserving `release_approval_evidence_id` and `target_validation_evidence_id`;
8. creates an opaque `NativeWebPublicationReceipt` with deterministic field and receipt-ID ordering.

The finalizer does not render learning content, validate an xAPI profile, persist or upload artifacts, or establish interoperability certification.

## Deterministic result and traceability

`PublicationMetadata` records release and target authority evidence identities alongside release/contract/version/standard/source/locale authority. Its fields, `PublicationOutcome`, and `NativeWebPublicationReceipt` remain externally read-only. The receipt carries both release-approval and target-validation evidence identities across the byte-finalization boundary.

`PublicationOutcome::canonical_json()` emits fixed field order and already-canonical blocker order. `NativeWebPublicationReceipt::canonical_json()` emits fixed field order, byte-derived hashes, and lexically canonical validation-receipt ordering. Exact release-byte equality is established only at finalization, not at admission.

## Failure model

Admission fails closed with typed errors for unavailable release/compatibility evidence, release/target authority identity mismatch, compatibility evidence bound to another release or source hash, unapproved release, malformed source identity, cross-target contract, missing authority fields, and duplicate blockers. A caller cannot manufacture compatibility by setting `approved=true`, supplying an empty blocker vector, or replaying cached target evidence for another immutable release.

Native finalization fails closed with typed `NativeWebPublicationError` values for incompatible admission, wrong publisher contract, source-byte mismatch, empty artifact/build-manifest bytes, and empty or duplicate validation-receipt identities.

## Test-first and stack evidence

The authority-port and exact-release-binding repairs on the parent publication-admission PR remain test-first. Commit `c8346a5fb1652c02a515b826f856a83e2072ae63` added regressions for target evidence produced for another release identity or source hash before production commit `f00ebc1523a682d35307ea4b14593378d1b8d190` introduced `CompatibilityReleaseIdentity` and typed mismatch failures.

Native byte-finalization expectations were committed before the original finalizer at `2534b18c3422e6f353cc74c3443f8834d4f58cd2`; they cover known SHA-256 fixtures, deterministic receipt ordering, source mismatch, incompatible/cmi5 rejection, zero-byte emitted payload rejection, and validation-receipt identity failures. The digest-case repair was also test-first: `f0ac99ce16762638e76a5983892c5785db4f36b0` established equivalent-case receipt expectations before `084aa22868f32157d13e63c99c9defbe6e9ac34a` canonicalized byte-derived receipt identity.

After the parent authority API advanced, the child was adapted on its existing writer branch: `d50546eaa28a18756f0a5cff4520e4575e27f274` combined the authority-bound admission model with byte finalization, `61a310ab07bece0ba6e946eff15298eff1835318` adapted native tests without removing prior edge cases, and merge commit `291100ae3ae5432894288fea266122eac5ca8686` non-destructively restacked the child on parent exact head `c51d1838f50fe758e63c1838f3cb1f8377a75fe7` without force-push or destructive rebase. This stack adaptation preserves already-tested finalizer behavior; it does not claim a new untested behavior change.

## Coverage contract

Repository CI runs rustfmt, Clippy with warnings denied, all-target tests, and rustdoc warnings-as-errors on `ubuntu-24.04`. Coverage uses pinned `cargo-llvm-cov` 0.9.0 plus exact `nightly-2026-08-30` branch instrumentation. The gate parses LLVM per-file summaries and requires each repository `src/` production file to have 100% line coverage and 100% branch coverage; test-only files cannot offset uncovered production paths. The gate also requires a nonzero production branch denominator.

## Security and operability

The crate forbids unsafe Rust and has no filesystem, secret, or network handling in these domain services. RustCrypto `sha2` is the only production dependency added by native finalization and remains subject to protected Dependency Review, SAST, and security evidence before integration. Authority-port adapters are privileged boundaries and require least privilege/auditability when persistence or remote services arrive. A missing or unavailable dependency-security signal is fail-closed, never translated into green.

## Future boundaries

A complete native renderer remains gated on a released shared xAPI 2.0 contract from `ContextualWisdomLab/learning-interoperability-contracts`. Durable persistence follows with 3NF two-or-more-word `snake_case` objects, append-only immutable `content_release` / `publication_receipt` authority, explicit transaction/audit semantics, and item-level UPSERT only for mutable indexes with tested idempotency. A service/API boundary is introduced only when durable storage or remote publishing requires one; then async handling, compose deployability, observability/recovery, and k6 evidence become mandatory.
