# Product and technical gap baseline

## Product responsibility

Learning Content Studio is the ContextualWisdomLab LCMS and authoring authority. It owns mutable authoring state, reusable learning objects and assets, review/approval, accessibility/localization/rights evidence, immutable `content_release` authority, publication admission, and deterministic target-specific publication evidence. Enrollment/completion, xAPI record truth, psychometric response/scoring truth, and payment truth remain outside this bounded context.

## Exact-head evidence contract

This baseline applies to the exact Git commit that contains it. GitHub PR and branch metadata are the authority for the current SHA; predecessor workflow, review, or merge-candidate evidence is never promoted to a successor head. The active stack is `agent/bootstrap-learning-content-studio` -> `agent/publishing-admission-kernel` -> `agent/native-web-artifact-projection`.

Live evidence re-fetched during the current commercialization iteration:

- the repository is public, organization-owned, and `fork=false`;
- parent PR #1 is open, Ready, and mechanically mergeable at exact head `7b8e0472451ab095301b7a5e40b1f99cd8af584b`; all observed inline review threads are resolved and CodeRabbit/Devin status contexts are successful;
- parent exact-head Learning Content Studio Quality run `33550893313` and SAST Semgrep run `33550893331` completed successfully;
- parent exact-head Security Scan run `33550893471` failed only in `dependency-review`: exact checkout succeeded, OSV/Trivy/Scorecard succeeded, and the authoritative dependency-graph comparison returned HTTP 403. ContextualWisdomLab/.github issue #810 already owns this fail-closed availability/configuration incident; no leaf-repository gate weakening is authorized;
- the live organization ruleset targets the protected default branch, requires one approving review and resolved review threads, and injects the central required workflow set. No self-approval, routine admin bypass, synthetic approval, or ruleset weakening is accepted;
- this PR was non-destructively restacked on the current PR #1 head with a regular two-parent merge. The parent advance added only `docs/index.md`; the post-restack compare shows the implementation branch is ahead of the current parent with no missing parent commit;
- the restack triggered a fresh exact-head Learning Content Studio Quality run. Until that run is terminal, predecessor repository-quality success remains lineage only;
- downstream PR #7 already implements the next byte-finalization slice and is restacked only after this parent moves; each downstream head must regenerate its own exact-head checks/reviews.

## Current feature specification

### Publication Admission bounded context

`src/lib.rs` implements a Rust fail-closed admission kernel for one approved immutable release and one selected publisher target.

Ubiquitous language:

- **content release**: approved immutable source authority presented to a publisher;
- **publisher target**: a delivery family with its own interoperability contract;
- **publisher contract**: version-specific transformation/validation boundary owned by one target;
- **blocking feature**: source semantic that cannot be preserved by the selected target;
- **publication admission**: deterministic decision allowing target transformation or returning incompatibility evidence;
- **trusted publication outcome**: immutable result that can only be constructed after admission invariants succeed;
- **native-web publication receipt**: downstream byte-bound evidence produced only after exact release-byte verification.

Admission invariants:

- `PublicationRequest` is the minimal transaction boundary for one release-target decision;
- release approval is mandatory;
- `source_hash` is required and non-blank before SHA-256 syntax validation;
- missing/whitespace-only `source_hash` returns `EmptyRequiredField("source_hash")`; non-empty malformed digests return `InvalidSourceHash`;
- `native_web_publisher` admits only `native_cwl_xapi_2_0/v1`;
- `cmi5_quartz_publisher` admits only `cmi5_quartz_xapi_1_0_3/v1`;
- cross-target contract selection fails closed;
- blocking features sort by `feature_code`, `source_component_reference`, then `reason_code`;
- duplicate blocking-feature triples are invalid;
- compatible outcomes have zero blockers; incompatible outcomes have at least one;
- `PublicationOutcome` and `PublicationMetadata` have no external write/constructor authority; downstream callers cannot manufacture a trusted compatible result;
- canonical JSON is deterministic after validation.

Review-discovered defects were repaired test-first: canonical blocker ordering, forged trusted outcomes/metadata, missing-vs-malformed `source_hash`, and mandatory 100% line/branch coverage. The source-hash regression was introduced at `cbecbcf98a7426051c343c956346906221d4df1e` before production repair `2cb6e32f548f960349902e2a0d18a2c6e78854af`.

## Domain-driven design

- **Core subdomain — Content Authoring & Release:** mutable authoring projects, revisions, approvals, and immutable releases.
- **Supporting subdomain — Publication Admission & Projection:** compatibility decisions, target-specific transformation boundaries, and byte-provenance receipts.
- **Supporting subdomain — Rights & Accessibility Evidence:** release-gating evidence referenced by immutable identity.
- **Generic subdomain — Artifact Storage / Delivery:** object storage, CDN, registry, telemetry, and deployment behind explicit ports/ACLs.

The context map is:

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
          +----> Native Web Renderer ----> Native Web Finalization
          +----> cmi5 Quartz adapter
          +----> later versioned adapters
                          |
                          v
                 Artifact Storage / Delivery
```

The publisher boundary is an anti-corruption layer. xAPI/cmi5/SCORM/Common Cartridge/QTI semantics must not become canonical authoring entities, and shared learning-interoperability contracts must be consumed from released ContextualWisdomLab/learning-interoperability-contracts schemas rather than copied locally.

Current aggregate/value-object direction: future `ContentProject` and immutable `ContentRelease` aggregate roots; current `PublicationRequest`, `PublicationMetadata`, `BlockingFeature`, and opaque `PublicationOutcome` value/domain objects; `evaluate_publication` as the current domain service. Domain events such as `content_release_approved`, `publication_admitted`, `publication_rejected`, and `native_publication_finalized` are deferred until durable transaction semantics exist.

## Commercialization gaps

| Gap | Owner | Live evidence | Action/state | Next verification |
| --- | --- | --- | --- | --- |
| Documentation-only foundation | Learning Content Studio | PR #1 has no installable application/service/publisher | **Partially repaired:** PR #6 adds executable Rust admission | Fresh exact-head fmt/clippy/test/rustdoc/coverage and independent review |
| Trusted outcome forgery | Learning Content Studio | review showed public variants/metadata could bypass admission | **Repaired test-first** with opaque trusted outcome/metadata | Fresh exact-head tests + reviewer confirmation |
| Missing source identity misclassified | Learning Content Studio | review showed blank `source_hash` became `InvalidSourceHash` | **Repaired test-first** | Fresh exact-head tests + reviewer confirmation |
| Canonical blocker order depended on caller order | Learning Content Studio | review demonstrated equivalent evidence serialized differently | **Repaired test-first** | Fresh exact-head tests + reviewer confirmation |
| Coverage mandate unenforced | Learning Content Studio | earlier quality lane ran tests without coverage proof | **Repaired:** pinned `cargo-llvm-cov` 0.9.0 + exact nightly branch instrumentation | Fresh exact-head line/branch gate must reach 100% |
| Dependency Review authoritative evidence unavailable | ContextualWisdomLab/.github / GitHub security configuration | PR #1 exact Security Scan returned HTTP 403 on dependency-graph compare while sibling scanners passed | **Fail-closed; canonical incident #810** | Authorized owner/configuration repair, then unchanged-head HTTP 200 canary where pinned dependency-review action executes |
| Stacked central review evidence | ContextualWisdomLab/.github | org ruleset injects required workflows only on protected default branch | **No local substitute:** use central stacked-review path, then retarget after parent integration | Current-head OpenCode/Noema evidence or protected retarget |
| Native byte-finalization not protected-integrated | Learning Content Studio | downstream PR #7 implements exact-byte SHA-256 receipts but remains stacked | **In progress downstream** | Restack on every parent move; exact-head checks/reviews; protected integration after #6 |
| No actual native renderer/package generator | Learning Content Studio + learning-interoperability-contracts | finalizer accepts already-emitted bytes and claims no renderer/conformance | Open/dependency-gated | Release shared xAPI 2.0 contract, implement deterministic renderer/manifest builder, feed exact bytes to finalizer, prove byte-identical fixtures |
| No immutable release/publication persistence | Learning Content Studio | no schema/migration/repository implementation | Open | 3NF append-only release/publication authority with explicit transaction boundaries and tested idempotency for mutable indexes only |
| No buyer-facing authoring workflow | Learning Content Studio | no application/UI/Storybook/Figma evidence | Open | Implement review -> accessibility/rights -> approval -> release workflow with accessibility and error-recovery evidence |
| No operability/deployment baseline | Learning Content Studio | no service/container/runtime | Open | Add service only when storage/publisher workflow requires it; then compose, observability, recovery, and k6 evidence |
| No public product release | Learning Content Studio | no release/tag/package | Open | Protected integration, SBOM/provenance, reproducible artifacts, public API maturity, then release |
| CEFR assessment-content vertical | Learning Content Studio + learning-interoperability-contracts | shared contract is not yet released | Dependency-gated | Consume released `cwl_cefr_language_assessment/v1`; do not duplicate schema authority |

## Persistence and data design guardrails

No database is introduced by the current admission kernel. Future authoritative relational objects use two-or-more-word `snake_case` names and remain in 3NF. Candidate objects include `content_release`, `release_component`, `release_asset`, `release_approval`, and `publication_receipt`; a generic one-word persistence object such as a table named `id` is invalid. Immutable release/publication facts are append-only. Item-level UPSERT is reserved for explicitly mutable authoring/index facts with tested idempotency keys and must never overwrite immutable authority. Read/write separation or materialized projections are introduced only from measured contention/load evidence.

## Security, compliance, and privacy

The current kernel processes identifiers and compatibility evidence and requires no PII. Dependency evidence remains fail-closed. Future reviewer/author identity, rights, consent, and publication operations must use least privilege, auditability, retention controls, encryption, and recovery evidence appropriate to CSAP/SOC 2 design objectives. Real people/institutions remain excluded from tests/docs.

## Verification matrix

- behavior changes are test-first;
- Rust owns deterministic publication math/hash-sensitive core logic;
- production consumes no synthetic demo data;
- public Rust API documentation is enforced by `missing_docs = "deny"` plus rustdoc warnings-as-errors;
- repository CI enforces formatting, Clippy with warnings denied, all-target tests, 100% line and nonzero 100% branch coverage, and rustdoc;
- central exact-head Security/SAST/review evidence remains mandatory for protected integration and is never replaced by repository-local green checks;
- every writer branch is re-fetched before a commit/ref mutation; restacks use ordinary merge ancestry without force-push or destructive rebases.

## Next bounded commercialization slice

Keep downstream PR #7 byte-finalization aligned with this exact parent, then unblock the true shared-contract owner before implementing the native renderer. After byte finalization is protected-integrated, the next repository-local slice is durable append-only publication receipt persistence with explicit release identity, contract identity, artifact/build-manifest hashes, validation-receipt identities, and transaction/audit semantics. Do not introduce mutable UPSERT over immutable release or publication authority.