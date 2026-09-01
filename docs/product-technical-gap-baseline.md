# Product and technical gap baseline

## Product responsibility

Learning Content Studio is the ContextualWisdomLab LCMS and authoring authority. It owns mutable authoring state, reusable learning objects/assets, review and approval, accessibility/localization/rights evidence, immutable `content_release` authority, publication admission, target projection, and publication provenance. Enrollment/completion, xAPI record truth, psychometric response/scoring truth, and payment truth remain outside this bounded context.

## Exact-head evidence contract

This baseline applies to the exact Git commit that contains it. GitHub PR/branch metadata is canonical for live SHA identity; predecessor workflow/review/merge-candidate evidence never transfers to a successor. The active stack is `agent/bootstrap-learning-content-studio` -> `agent/publishing-admission-kernel` -> `agent/native-web-artifact-projection`.

Live evidence re-fetched in the current commercialization iteration:

- repository is public, organization-owned, and `fork=false`;
- PR #1 is open, Ready, mechanically mergeable at `7b8e0472451ab095301b7a5e40b1f99cd8af584b`; observed review threads are resolved; repository Quality and SAST Semgrep completed successfully on that exact head;
- PR #1 exact-head Security Scan failed only in Dependency Review after the exact checkout succeeded: OSV, Trivy, and Scorecard succeeded, while the authoritative dependency-graph comparison returned HTTP 403. `ContextualWisdomLab/.github#810` is the canonical fail-closed availability/configuration incident. No leaf workaround, scanner substitution, or gate weakening is authorized;
- the live organization ruleset targets the protected default branch, requires one approval plus resolved review threads, and injects central required workflows. No self-approval, routine administrator bypass, synthetic approval, or protection weakening is accepted;
- PR #6 was non-destructively restacked on the current PR #1 branch with a regular two-parent merge, then refreshed its commercialization baseline. Its current parent identity for this stack is `c1364390a31a87ef02d0cc23013e1e7b65ec7544`; the fresh repository Quality run on that head is queued and predecessor success remains lineage only;
- this branch previously restacked on the prior PR #6 parent and is now being converged again onto the refreshed parent. Its containing commit, not any SHA written in prose, is authoritative for PR #7 exact-head verification;
- all new exact-head repository checks and independent review evidence must regenerate after this convergence.

## Feature specification

### Publication Admission

`evaluate_publication` is the Rust domain service that decides whether one approved immutable release may proceed through one target contract.

Admission ubiquitous language and invariants:

- **content release** — approved immutable source authority;
- **publisher target** — delivery family with its own interoperability contract;
- **publisher contract** — versioned transformation/validation boundary owned by one target;
- **blocking feature** — source semantic that the selected target cannot preserve;
- **trusted publication outcome** — opaque result mintable only after admission invariants succeed;
- release approval is mandatory;
- `source_hash` is required and non-blank before SHA-256 syntax validation;
- missing/whitespace-only `source_hash` returns `EmptyRequiredField("source_hash")`; non-empty malformed input returns `InvalidSourceHash`;
- `native_web_publisher` admits only `native_cwl_xapi_2_0/v1`;
- `cmi5_quartz_publisher` admits only `cmi5_quartz_xapi_1_0_3/v1`;
- cross-target contract fallback fails closed;
- blocking features sort by `feature_code`, `source_component_reference`, then `reason_code`; exact duplicate triples are invalid;
- `PublicationOutcome` and `PublicationMetadata` expose no external constructor/write authority;
- canonical JSON has deterministic field/array ordering.

### Native-Web Publication Finalization

`finalize_native_web_publication` is the next Rust trust boundary after a native renderer has already emitted bytes. It proves provenance; it does not render content or certify xAPI conformance.

Finalization invariants:

- only a trusted compatible outcome owned by `native_cwl_xapi_2_0/v1` is accepted;
- SHA-256 is recomputed from the exact canonical immutable release bytes and compared fail-closed with admitted source identity;
- upper/lower hexadecimal spellings compare as the same digest, but receipt `source_hash` is the canonical lowercase digest recomputed from exact bytes, so equivalent admission spelling cannot alter final evidence;
- empty canonical release bytes are valid when their SHA-256 identity matches; empty emitted artifact bytes or empty build-manifest bytes are invalid;
- `artifact_hash` and `build_manifest_hash` are computed inside the trusted finalizer from exact emitted bytes rather than caller assertions;
- validation-receipt identifiers are opaque external identities: each must be non-empty, is preserved exactly, sorts lexically, and exact duplicates are rejected;
- `NativeWebPublicationReceipt` is opaque externally and emits deterministic canonical JSON;
- production uses pinned RustCrypto `sha2` 0.11.0 in safe Rust; dependency/security evidence remains mandatory;
- a receipt is byte-provenance evidence only, not a renderer result, xAPI 2.0 conformance claim, certification, persistence record, or deployment proof.

Test-first repair lineage includes:

- admission canonical blocker ordering, forged trusted outcome/metadata, source-hash missing-vs-malformed classification, and coverage-gate regressions;
- native finalization behavior/edge cases introduced before production at `2534b18c3422e6f353cc74c3443f8834d4f58cd2`;
- equivalent digest-casing receipt regression `f0ac99ce16762638e76a5983892c5785db4f36b0`, followed by production canonicalization `084aa22868f32157d13e63c99c9defbe6e9ac34a`;
- exact-head rustfmt failure repaired without behavior change at `3242cf0b7064bdcc399e48e40b9bc055f180d973`;
- `sha2` 0.11 digest `LowerHex` compile failure repaired at `e0edf3b86c264993644566fbad0681b5796e4642` by explicit lowercase byte serialization while preserving the prior known-hash tests.

## Domain-driven design

- **Core subdomain — Content Authoring & Release:** mutable projects/revisions/review/approval and immutable release authority.
- **Supporting subdomain — Publication Admission & Projection:** target compatibility, transformation boundary, and byte-bound publication evidence.
- **Supporting subdomain — Rights & Accessibility Evidence:** release-gating evidence referenced by immutable identity and publication receipts.
- **Generic subdomain — Artifact Storage / Delivery:** object storage, CDN, registry, telemetry, and deployment behind explicit ports/ACLs.

Context map:

```text
Authoring Surface (Inkspan ACL)
          |
          v
Content Authoring & Release ----> Rights & Accessibility Evidence
          |
          | approved immutable content_release
          v
Publication Admission
          |
          +----> Native Web Renderer (released shared xAPI contract)
          |              |
          |              v
          |       Native Web Finalization
          |
          +----> cmi5 Quartz adapter
          +----> later versioned adapters
                         |
                         v
                Artifact Storage / Delivery
```

The publisher boundary is an anti-corruption layer. xAPI/cmi5/SCORM/Common Cartridge/QTI semantics cannot become canonical authoring entities. A native renderer must consume a released shared schema from `ContextualWisdomLab/learning-interoperability-contracts`; it must not copy that repository's contract authority.

Current domain/value objects are `PublicationRequest`, `PublicationMetadata`, `BlockingFeature`, opaque `PublicationOutcome`, and opaque `NativeWebPublicationReceipt`. Current domain services are `evaluate_publication` and `finalize_native_web_publication`. Future aggregate roots include `ContentProject` and immutable `ContentRelease`; durable domain events such as `content_release_approved`, `publication_admitted`, `publication_rejected`, and `native_publication_finalized` wait for explicit persistence transaction semantics.

## Commercialization gaps

| Gap | Owner | Evidence | Action/state | Next verification |
| --- | --- | --- | --- | --- |
| Foundation has no installable buyer workflow | Learning Content Studio | PR #1 explicitly remains pre-release/documentation foundation | **Partially repaired:** executable admission + native byte-finalization kernels exist in stacked PRs | Exact-head quality/security/review plus protected integration |
| Trusted admission could be forged | Learning Content Studio | review exposed public outcome/metadata authority | **Repaired test-first** | Fresh exact-head tests/rustdoc/reviewer confirmation |
| Missing source identity misclassified as malformed | Learning Content Studio | review reproduced blank digest classification defect | **Repaired test-first** | Fresh exact-head tests/reviewer confirmation |
| Receipt evidence depended on equivalent digest casing | Learning Content Studio | PR #7 review reproduced different receipt evidence | **Repaired test-first:** `f0ac99ce...` -> `084aa228...` | Fresh exact-head tests/coverage/reviewer confirmation |
| Exact-byte publication provenance | Learning Content Studio | admission alone cannot prove byte identity | **Implemented in PR #7:** recompute release/artifact/manifest SHA-256 and mint opaque receipt | Fresh exact-head fmt/clippy/test/100%-coverage/rustdoc/security/review |
| Dependency Review evidence unavailable | ContextualWisdomLab/.github / GitHub configuration | PR #1 exact compare returned HTTP 403 | **Fail-closed; canonical incident #810** | Authorized owner/configuration repair; unchanged-head HTTP 200 canary where pinned action actually executes |
| Stacked central review evidence | ContextualWisdomLab/.github | central required-workflow rules target protected default branch | **No local substitute** | Central stacked review or protected retarget after parent integration |
| Shared native xAPI 2.0 contract not released | learning-interoperability-contracts | PR #7 deliberately claims no renderer/conformance without released shared contract | Dependency-gated | Repair/release true owner contract before native renderer claim |
| No actual native renderer/package generator | Learning Content Studio | finalizer consumes already-emitted bytes | Open | Implement deterministic renderer/manifest builder against released shared contract; byte-identical fixtures; feed exact bytes into finalizer |
| No immutable release/publication persistence | Learning Content Studio | no schema/migration/repository | Open | 3NF append-only release/publication authority with explicit transaction/audit semantics |
| No buyer-facing authoring UX | Learning Content Studio | no application/UI/Storybook/Figma evidence | Open | Review -> accessibility/rights -> approval -> release flow; accessibility/touch/error-recovery verification |
| No operability/deployment baseline | Learning Content Studio | no service/container/runtime | Open | Add service only when persistence/publisher workflow requires it, then compose/observability/recovery/k6 evidence |
| No public product release | Learning Content Studio | no protected release/tag/package | Open | Protected integration, SBOM/provenance, reproducible artifacts, public API maturity, then release |
| CEFR vertical | Learning Content Studio + learning-interoperability-contracts | shared assessment-content contract not released | Dependency-gated | Consume released `cwl_cefr_language_assessment/v1`; do not duplicate schema authority |

## Persistence and data design guardrails

No database is introduced by these kernels. Future authoritative relational objects use at least two semantic words, `snake_case`, and 3NF; candidate objects include `content_release`, `release_component`, `release_asset`, `release_approval`, and `publication_receipt`. A generic one-word persistence object such as a table named `id` is invalid. Immutable release/publication facts are append-only. Item-level UPSERT is permitted only for explicitly mutable authoring/index facts with tested idempotency keys and must never overwrite immutable authority. Read/write separation or materialized projections require measured contention/load evidence rather than pre-emptive denormalization.

## Security, compliance, and privacy

Current kernels operate on identifiers, compatibility evidence, and byte buffers and require no PII. SHA-256 is used as immutable byte identity, not as a password function or confidentiality mechanism. Future reviewer/author identity, rights/consent, and publication operations require least privilege, auditability, retention, encryption, and recovery evidence appropriate to CSAP/SOC 2 design objectives. Real people/institutions remain excluded from fixtures/docs.

## Verification matrix

- behavior changes are test-first;
- deterministic hash-sensitive publication computation remains Rust;
- production consumes no synthetic demo data;
- `missing_docs = "deny"` plus rustdoc warnings-as-errors enforce documented public Rust APIs;
- repository CI enforces rustfmt, Clippy with warnings denied, all-target tests, 100% line coverage, nonzero 100% branch coverage, and rustdoc;
- exact-head central Security/SAST/review evidence remains mandatory for protected integration and is never replaced by repository-local green checks;
- writer branches are re-fetched before ref mutation; stack convergence uses ordinary merge ancestry without force-push or destructive rebase.

## Next bounded commercialization slice

First obtain a released shared native xAPI 2.0 contract from its true owner. Then implement the deterministic native renderer/manifest builder and feed its exact output bytes into the existing finalizer. In parallel, the next repository-local infrastructure slice is append-only `content_release` / `publication_receipt` persistence with explicit release, contract, artifact, build-manifest, validation-receipt, transaction, and audit identities. Do not introduce mutable UPSERT over immutable release/publication authority.