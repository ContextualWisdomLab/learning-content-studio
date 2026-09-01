# Product and technical gap baseline

## Product responsibility

Learning Content Studio is the ContextualWisdomLab LCMS and authoring authority. It owns mutable authoring state, review/approval, accessibility/localization/rights evidence, immutable `content_release` authority, publication admission, target projection, and publication provenance. Enrollment/completion, xAPI record truth, psychometric response/scoring truth, and payment truth remain outside this bounded context.

## Exact-head evidence contract

This baseline applies to the exact Git commit that contains it. GitHub PR/branch metadata is canonical for live SHA identity; predecessor workflow/review evidence never transfers to a successor head.

Live commercialization evidence:

- repository is public, organization-owned, and `fork=false`;
- PR #1 is open/Ready/mechanically mergeable at `7b8e0472451ab095301b7a5e40b1f99cd8af584b`; exact-head repository Quality and SAST succeeded;
- PR #1 Security Scan failed only in Dependency Review after exact checkout; OSV/Trivy/Scorecard succeeded and the authoritative dependency-graph compare returned HTTP 403. `ContextualWisdomLab/.github#810` remains the canonical fail-closed configuration/availability incident;
- the organization ruleset requires one approval and resolved threads on the protected default branch; no self-approval, admin bypass, or protection weakening is accepted;
- PR #6 was non-destructively restacked on the current parent. A subsequent exact-head Devin review found two real gaps: caller-controlled approval/blocker assertions could mint an opaque but falsely trusted compatible outcome, and aggregate coverage could let test-only code mask uncovered production paths;
- authority-port regression tests were committed first at `77d6d9b27d42f16d379fc427ff61e36e7b443f85` before production repair;
- the production repair removes approval/source/locale/contract/version/standard/blockers from `PublicationRequest`, introduces `ReleaseAuthorityPort` and `TargetCompatibilityPort`, cross-binds their evidence to caller release/target intent, and records stable release-approval/target-validation evidence identities;
- the coverage repair evaluates LLVM per-file summaries for repository `src/` production files and requires every production line/branch to be covered, with a nonzero production branch denominator;
- downstream PR #7 must be restacked again after this parent repair and revalidated on its own exact head.

## Feature specification and ubiquitous language

- **publication request**: caller intent containing only release identity and publisher target;
- **release authority evidence**: immutable release identity, source SHA-256, locale, approval state, and stable approval-evidence identity supplied by `ReleaseAuthorityPort`;
- **target compatibility evidence**: target, contract/version/standard, validation-evidence identity, and blockers supplied by `TargetCompatibilityPort`;
- **blocking feature**: authority-owned evidence that target transformation would lose semantics;
- **publication admission**: deterministic cross-binding and validation of caller intent against both authority ports;
- **publication outcome**: opaque compatible/incompatible result that preserves authority traceability;
- **native-web publication receipt**: downstream byte-bound evidence after exact release/artifact/manifest hashing.

Admission invariants:

- caller cannot assert approval or omit blockers through `PublicationRequest`;
- release evidence must exist and exactly match the requested release identity;
- authoritative approval must be true;
- source hash, locale, and approval-evidence identity are required; SHA-256 syntax is validated;
- target evidence must exist and match the requested target;
- target-owned contract/version/standard/validation-evidence identities are required;
- target contract must match the target-specific required contract;
- blockers are validated, sorted by `feature_code`/`source_component_reference`/`reason_code`, and exact duplicates are rejected;
- compatible outcome requires zero authority-supplied blockers; incompatible outcome requires at least one;
- `PublicationOutcome` and `PublicationMetadata` remain externally read-only;
- exact byte equality is intentionally downstream: native finalization recomputes release/artifact/manifest SHA-256 from bytes.

## DDD context map

- **Core — Content Authoring & Release:** mutable projects/revisions/review/approval and immutable release authority.
- **Supporting — Publication Admission & Projection:** authority-backed compatibility decision, target transformation boundary, and byte provenance.
- **Supporting — Rights & Accessibility Evidence:** release and target gating evidence.
- **Generic — Artifact Storage / Delivery:** object storage/CDN/registry/telemetry/deployment behind ACLs.

`ReleaseAuthorityPort` and `TargetCompatibilityPort` are ACLs between owning bounded contexts and Publication Admission. Production implementations must not reconstruct authority from mutable request fields or synthetic/demo data.

## Commercialization gaps

| Gap | Owner | Evidence | Action/state | Next verification |
| --- | --- | --- | --- | --- |
| Caller assertions could forge trusted compatibility | Learning Content Studio | exact-head Devin review on PR #6 | **Repaired test-first** with authority ports and intent-only request | Exact-head fmt/clippy/tests/production coverage/rustdoc + reviewer confirmation |
| Aggregate coverage could mask production gaps | Learning Content Studio | exact-head Devin review on PR #6 | **Repaired** with per-`src/` line/branch enforcement | Exact-head coverage job proves every production file 100% |
| Dependency Review unavailable | ContextualWisdomLab/.github / GitHub configuration | PR #1 dependency compare HTTP 403 | **Fail-closed; #810 canonical** | Authorized configuration repair + unchanged-head HTTP 200 canary |
| Stacked central review | ContextualWisdomLab/.github | required workflows target protected default branch | No local approval substitute | Central stacked review or protected retarget after parent integration |
| Native byte finalization not integrated | Learning Content Studio | PR #7 implements exact-byte receipt evidence | In progress downstream | Restack on repaired PR #6, adapt API, rerun exact-head checks/reviews |
| Shared native xAPI 2.0 contract not released | learning-interoperability-contracts | renderer explicitly dependency-gated | Open upstream | Repair/release true owner before claiming native renderer conformance |
| No native renderer/package generator | Learning Content Studio | finalizer consumes already-emitted bytes | Open | Deterministic renderer/manifest builder against released shared contract + byte-identical fixtures |
| No immutable persistence | Learning Content Studio | no schema/migration/repository | Open | 3NF append-only `content_release`/`publication_receipt` authority and audit transactions |
| No buyer-facing authoring UX | Learning Content Studio | no app/UI/Storybook/Figma evidence | Open | Review -> accessibility/rights -> approval -> release workflow with accessibility/error-recovery evidence |
| No operability/deployment baseline | Learning Content Studio | no service/container/runtime | Open | Add only when remote persistence/publishing requires it; then compose/observability/recovery/k6 |
| No public product release | Learning Content Studio | no protected release/tag/package | Open | Protected integration, SBOM/provenance/reproducibility/API maturity |

## Persistence and security guardrails

Future authoritative relational objects use two-or-more-word `snake_case` names and 3NF. `content_release`, `release_component`, `release_asset`, `release_approval`, and `publication_receipt` are append-only where immutable. Item-level UPSERT is allowed only for explicitly mutable indexes with tested idempotency keys. Authority-port adapters are privileged security boundaries and require least privilege, auditability, retention, encryption, and recovery evidence appropriate to CSAP/SOC 2 design goals. Current kernels require no PII; fixtures/docs use no real persons/institutions.

## Verification matrix

- behavior/security trust change is test-first (`77d6d9b2...` RED before production repair);
- deterministic/hash-sensitive core logic remains Rust;
- production consumes no synthetic demo data;
- public Rust APIs use `missing_docs = "deny"` plus rustdoc warnings-as-errors;
- CI requires rustfmt, Clippy `-D warnings`, all-target tests, and 100% per-production-file line/branch coverage with nonzero production branch evidence;
- central exact-head Security/SAST/review evidence remains mandatory and cannot be replaced by repository-local green checks;
- writer branches are re-fetched before mutation; stack updates use ordinary merge ancestry without force-push/destructive rebase.

## Next bounded commercialization slice

After exact-head validation of the authority-port repair, restack/adapt PR #7 to the new admission API. Then fix/release the shared native xAPI 2.0 contract in its true owner before implementing a conformant native renderer. The next repository-local infrastructure slice is append-only release/publication receipt persistence with explicit authority/evidence/audit transaction identities.