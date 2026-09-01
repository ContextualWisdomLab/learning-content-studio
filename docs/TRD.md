# Technical requirements — Publication Admission

## Scope

This TRD covers the first executable Learning Content Studio trust boundary. It evaluates caller intent for one immutable release/target only after obtaining approval/release evidence from the release authority and semantic compatibility evidence from the target-validation authority. Package generation, persistence, network services, and UI remain outside this slice.

## Runtime and API

The implementation is a dependency-free Rust library crate in `src/lib.rs` with `unsafe_code = "forbid"` and `missing_docs = "deny"`.

Primary API:

```text
evaluate_publication(
    PublicationRequest,
    &dyn ReleaseAuthorityPort,
    &dyn TargetCompatibilityPort,
) -> Result<PublicationOutcome, AdmissionError>
```

`PublicationRequest` contains only caller intent: `content_release_id` and `PublisherTarget`. It has no caller-controlled approval boolean, source hash, locale, publisher contract/version/standard, or blocker vector.

`ReleaseAuthorityPort` owns immutable-release lookup. Its `ReleaseAuthorityEvidence` supplies release identity, SHA-256 source identity, locale, approval state, and stable approval-evidence identity. `TargetCompatibilityPort` owns target validation. Its `TargetCompatibilityEvidence` supplies a `CompatibilityReleaseIdentity` containing the exact immutable `content_release_id` and `source_hash` validated, plus target, contract/version/standard, stable validation-evidence identity, and zero or more blockers. Production adapters are security-sensitive ACLs and must derive evidence from authoritative stores/services, not request fields or synthetic/demo state.

## Validation order

1. reject an empty requested release identity;
2. require release-authority evidence and reject a returned release identity that does not exactly match the request;
3. reject release authority evidence that is not approved;
4. validate authority-owned source hash, locale, and approval-evidence identity;
5. validate SHA-256 syntax (byte equality is verified later by the byte-owning finalizer);
6. require target-compatibility evidence and reject evidence for another target;
7. reject target compatibility evidence whose `content_release_id` does not exactly match release-authority evidence;
8. reject target compatibility evidence whose `source_hash` does not exactly match release-authority evidence, so stale cached validation cannot authorize different immutable bytes;
9. validate target-owned contract/version/standard/validation-evidence identities;
10. reject a contract not owned by the requested target;
11. validate blocker identities, sort by `feature_code`, `source_component_reference`, `reason_code`, and reject exact duplicates;
12. mint an opaque compatible outcome only when the authority-supplied blocker set is empty; otherwise mint an incompatible outcome.

No mutable authoring state, caller approval claim, caller blocker omission, wall clock, environment locale, network-fetched content, random ID, or process ordering participates in the decision.

## Deterministic result and traceability

`PublicationMetadata` records release and target authority evidence identities alongside release/contract/version/standard/source/locale authority. Its fields and `PublicationOutcome` fields are private. Read-only accessors preserve traceability while preventing downstream mutation.

`PublicationOutcome::canonical_json()` emits fixed field order and already-canonical blocker order. Admission evidence does not prove release-byte equality; the downstream native byte finalizer recomputes SHA-256 over exact immutable release bytes before minting artifact provenance.

## Failure model

Validation fails closed with typed errors for unavailable release/compatibility evidence, release/target authority identity mismatch, compatibility evidence bound to another release or source hash, unapproved release, malformed source identity, cross-target contract, missing authority fields, and duplicate blockers. A caller cannot manufacture compatibility by setting `approved=true`, supplying an empty blocker vector, or replaying cached target evidence for another immutable release.

## Test-first evidence

`tests/authority_ports.rs` was committed in RED state before the authority-port production repair. It requires authority-owned approval/blocker truth, release/target cross-binding, and fail-closed unavailable evidence.

The release-binding repair was also test-first. Commit `c8346a5fb1652c02a515b826f856a83e2072ae63` added `tests/compatibility_release_binding.rs` with failing expectations for target evidence produced for another release identity or source hash. Production commit `f00ebc1523a682d35307ea4b14593378d1b8d190` introduced `CompatibilityReleaseIdentity` and typed `CompatibilityReleaseMismatch` / `CompatibilitySourceMismatch` failures before the test fixtures were adapted to the new constructor.

`tests/publication_admission.rs` additionally covers required identity failures, both target contracts, SHA-256 syntax branches, order-independent blockers, duplicate rejection, release-bound authority traceability accessors, compatible/incompatible canonical JSON, and every JSON control-character class.

## Coverage contract

Repository CI runs rustfmt, Clippy with warnings denied, all-target tests, and rustdoc warnings-as-errors on `ubuntu-24.04`. Coverage uses pinned `cargo-llvm-cov` 0.9.0 plus exact `nightly-2026-08-30` branch instrumentation. The gate parses LLVM per-file summaries and requires each repository `src/` production file to have 100% line coverage and 100% branch coverage; test-only files cannot offset uncovered production paths. The gate also requires a nonzero production branch denominator.

## Security and operability

The admission crate has no external package dependency, filesystem access, secret handling, or unsafe Rust. Authority-port adapters are explicit privileged boundaries and require least privilege/auditability when persistence or remote services arrive. Central Security/SAST/review workflows remain independent merge gates. Dependency Review HTTP 403 remains a fail-closed control-plane/configuration incident tracked in `ContextualWisdomLab/.github#810`; it is never translated into green.

## Future boundaries

The native byte-finalization slice consumes only a compatible opaque admission and exact immutable release/artifact/manifest bytes. A full native renderer remains gated on a released shared xAPI 2.0 contract from `ContextualWisdomLab/learning-interoperability-contracts`. Persistence follows with 3NF two-or-more-word `snake_case` objects, append-only immutable authority, explicit transaction/audit semantics, and item-level UPSERT only for mutable indexes. A service/API boundary is introduced only when durable storage or remote publishing requires one; then async handling, compose deployability, observability/recovery, and k6 evidence become mandatory.
