# Product and technical gap baseline

## Product responsibility

Learning Content Studio is the ContextualWisdomLab LCMS and authoring authority. It owns mutable authoring state, review/approval, accessibility/localization/rights evidence, immutable `content_release` authority, publication admission, target projection, and publication provenance. Enrollment/completion, xAPI record truth, psychometric response/scoring truth, and payment truth remain outside this bounded context.

## Exact-head evidence contract

This baseline applies to the exact Git commit that contains it. GitHub PR/branch metadata is canonical for live SHA identity; predecessor workflow/review evidence never transfers to a successor head.

Live commercialization evidence:

- repository is public, organization-owned, and `fork=false`;
- PR #1 is open/Ready/mechanically mergeable at `e7977e5736b425e6221481934b25811ab27d7557`; exact-head Security Scan `33569943004`, SAST Semgrep `33569943093`, and Learning Content Studio Quality `33569943228` are queued and therefore not passing evidence yet;
- the organization ruleset requires ordinary protected-branch governance, including independent approval; no self-approval, admin bypass, or protection weakening is accepted;
- PR #6 is the first executable Publication Admission kernel and remains stacked on PR #1. Earlier review gaps around caller-controlled approval/blockers and aggregate-only coverage were repaired with authority ports and per-production-file coverage enforcement;
- the latest live review exposed a separate trust defect: `TargetCompatibilityEvidence` was target-bound but not release-bound, so cached compatibility evidence for another immutable release could be replayed;
- release-binding regression commit `c8346a5fb1652c02a515b826f856a83e2072ae63` preceded production repair and required mismatched release identity and mismatched source hash to fail closed;
- production commit `f00ebc1523a682d35307ea4b14593378d1b8d190` adds `CompatibilityReleaseIdentity`, binds target evidence to exact `content_release_id` plus `source_hash`, and returns typed `CompatibilityReleaseMismatch` / `CompatibilitySourceMismatch` failures;
- all affected existing test fixtures were adapted without removing prior edge cases; exact-head repository and central checks must be re-established after the documentation commits that contain this baseline;
- downstream PR #7 must be revalidated against the release-bound admission API rather than assuming predecessor-head compatibility.

## Feature specification and ubiquitous language

- **publication request**: caller intent containing only release identity and publisher target;
- **release authority evidence**: immutable release identity, source SHA-256, locale, approval state, and stable approval-evidence identity supplied by `ReleaseAuthorityPort`;
- **compatibility release identity**: exact immutable `content_release_id` and source SHA-256 that the target validator actually evaluated;
- **target compatibility evidence**: release-bound target, contract/version/standard, validation-evidence identity, and blockers supplied by `TargetCompatibilityPort`;
- **blocking feature**: authority-owned evidence that target transformation would lose semantics;
- **publication admission**: deterministic cross-binding and validation of caller intent against release authority and release-bound target validation;
- **publication outcome**: opaque compatible/incompatible result that preserves authority traceability;
- **native-web publication receipt**: downstream byte-bound evidence after exact release/artifact/manifest hashing.

Admission invariants:

- caller cannot assert approval or omit blockers through `PublicationRequest`;
- release evidence must exist and exactly match the requested release identity;
- authoritative approval must be true;
- source hash, locale, and approval-evidence identity are required; SHA-256 syntax is validated;
- target evidence must exist and match the requested target;
- target evidence must identify the same `content_release_id` and exact `source_hash` as release-authority evidence; cached evidence for another release or source identity fails closed;
- target-owned contract/version/standard/validation-evidence identities are required;
- target contract must match the target-specific required contract;
- blockers are validated, sorted by `feature_code`/`source_component_reference`/`reason_code`, and exact duplicates are rejected;
- compatible outcome requires zero authority-supplied blockers; incompatible outcome requires at least one;
- `PublicationOutcome` and `PublicationMetadata` remain externally read-only;
- admission source-hash validation is identity/syntax plus authority cross-binding; exact byte equality is intentionally downstream, where native finalization recomputes release/artifact/manifest SHA-256 from bytes.

## DDD context map

- **Core — Content Authoring & Release:** mutable projects/revisions/review/approval and immutable release authority.
- **Supporting — Publication Admission & Projection:** authority-backed compatibility decision, exact release-to-validation binding, target transformation boundary, and byte provenance.
- **Supporting — Rights & Accessibility Evidence:** release and target gating evidence.
- **Generic — Artifact Storage / Delivery:** object storage/CDN/registry/telemetry/deployment behind ACLs.

`ReleaseAuthorityPort` and `TargetCompatibilityPort` are ACLs between owning bounded contexts and Publication Admission. `CompatibilityReleaseIdentity` is the value object that prevents target-validation evidence from crossing immutable release boundaries. Production implementations must not reconstruct authority from mutable request fields or synthetic/demo data.

## Commercialization gaps

| Gap | Owner | Evidence | Action/state | Next verification |
| --- | --- | --- | --- | --- |
| Cached target compatibility evidence could authorize another release | Learning Content Studio | live unresolved Devin finding on PR #6 verified against source | **Repaired test-first** by `c8346a5f...` then `f00ebc15...`; exact release/hash binding enforced | Exact-head fmt/clippy/tests/production coverage/rustdoc + reviewer confirmation |
| Caller assertions could forge trusted compatibility | Learning Content Studio | earlier PR #6 review | **Repaired test-first** with authority ports and intent-only request | Preserve under exact-head regression suite |
| Aggregate coverage could mask production gaps | Learning Content Studio | earlier PR #6 review | **Repaired** with per-`src/` line/branch enforcement | Exact-head coverage proves every production file 100% |
| Parent foundation not protected-integrated | Learning Content Studio / governance | PR #1 at `e7977e57...`; exact-head required workflows queued | Open without bypass | Unchanged-head required checks + independent approval + ordinary merge |
| Dependency Review availability/configuration | ContextualWisdomLab/.github / GitHub configuration | prior dependency compare HTTP 403; canonical `.github#810` | Fail closed | Authorized control-plane repair + exact-head canary |
| Stacked central review | ContextualWisdomLab/.github | protected-default workflow/review policy | No local approval substitute | Central stacked review or protected retarget after parent integration |
| Native byte finalization integration | Learning Content Studio | PR #7 owns exact-byte receipt evidence | In progress downstream | Revalidate/adapt PR #7 against release-bound PR #6 API |
| Shared native xAPI 2.0 contract not released | `ContextualWisdomLab/learning-interoperability-contracts` | renderer explicitly dependency-gated | Open upstream | Release true owner before native renderer conformance claim |
| No native renderer/package generator | Learning Content Studio | finalizer consumes already-emitted bytes | Open | Deterministic renderer/manifest builder against released shared contract + byte-identical fixtures |
| No immutable persistence | Learning Content Studio | no schema/migration/repository | Open | 3NF append-only `content_release`/`publication_receipt` authority and audit transactions |
| No buyer-facing authoring UX | Learning Content Studio | no app/UI/Storybook/Figma evidence | Open | Review -> accessibility/rights -> approval -> release workflow with accessibility/error-recovery evidence |
| No operability/deployment baseline | Learning Content Studio | no service/container/runtime | Open | Add when remote persistence/publishing requires it; then compose/observability/recovery/k6 |
| No public product release | Learning Content Studio | no protected release/tag/package | Open | Protected integration, SBOM/provenance/reproducibility/API maturity |

## Persistence and security guardrails

Future authoritative relational objects use two-or-more-word `snake_case` names and 3NF. `content_release`, `release_component`, `release_asset`, `release_approval`, and `publication_receipt` are append-only where immutable. Item-level UPSERT is allowed only for explicitly mutable indexes with tested idempotency keys. Authority-port adapters are privileged security boundaries and require least privilege, auditability, retention, encryption, and recovery evidence appropriate to CSAP/SOC 2 design goals. Current kernels require no PII; fixtures/docs use no real persons/institutions.

## Verification matrix

- latest trust repair is test-first: `c8346a5fb1652c02a515b826f856a83e2072ae63` establishes release/source mismatch expectations before production `f00ebc1523a682d35307ea4b14593378d1b8d190`;
- deterministic/hash-sensitive core logic remains Rust;
- production consumes no synthetic demo data;
- public Rust APIs use `missing_docs = "deny"` plus rustdoc warnings-as-errors;
- CI requires rustfmt, Clippy `-D warnings`, all-target tests, and 100% per-production-file line/branch coverage with nonzero production branch evidence;
- central exact-head Security/SAST/review evidence remains mandatory and cannot be replaced by repository-local green checks;
- writer branches are re-fetched before mutation; stack updates use ordinary ancestry without force-push/destructive rebase.

## Next bounded commercialization slice

First establish exact-head checks and independent review for the release-binding repair, then adapt/revalidate PR #7 against that API. After the publication trust stack is coherent, repair/release the shared native xAPI 2.0 contract in its true owner before implementing a conformant native renderer. The next repository-local infrastructure slice is append-only release/publication receipt persistence with explicit authority/evidence/audit transaction identities.
