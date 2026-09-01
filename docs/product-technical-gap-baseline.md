# Product and technical gap baseline

## Product responsibility

Learning Content Studio is the ContextualWisdomLab LCMS and authoring authority. It owns mutable authoring state, review/approval, accessibility/localization/rights evidence, immutable `content_release` authority, publication admission, target projection, byte-finalized publication evidence, and publication provenance. Enrollment/completion, xAPI record truth, psychometric response/scoring truth, and payment truth remain outside this bounded context.

## Exact-head evidence contract

This baseline applies to the exact Git commit that contains it. GitHub PR/branch metadata is canonical for live SHA identity; predecessor workflow/review evidence never transfers to a successor head.

Live commercialization evidence at the last pre-write re-fetch:

- repository is public, organization-owned, and `fork=false`;
- the active organization ruleset `CWL Central required workflows` applies to the default branch and requires one approving review, resolved review threads, and central close-empty/OpenCode/merge-scheduler/security/Strix/SAST/Noema workflows; no admin bypass or ruleset weakening is used by this work;
- PR #1 is open/Ready/mechanically mergeable at `e7977e5736b425e6221481934b25811ab27d7557`, but its exact-head central/repository workflows remain non-passing until queued execution and independent approval complete;
- PR #6 is open/Ready/mechanically mergeable at `c51d1838f50fe758e63c1838f3cb1f8377a75fe7`. Its release-binding review defect is repaired and the review thread is resolved; Quality run `33571176737` remains pending, and its observed job had no allocated runner or executed steps;
- PR #7 is non-destructively restacked on the current PR #6 head. Merge commit `291100ae3ae5432894288fea266122eac5ca8686` has both the child writer state and `c51d1838f50fe758e63c1838f3cb1f8377a75fe7` as parents, so no force-push or destructive rebase was used;
- the restacked PR #7 source and tests use authority-backed admission and preserve native exact-byte finalization. Pre-baseline head `29fdaec8761149c4f20cf5bfc08a13a6fdb46131` was open/Ready/mechanically mergeable; Quality run `33571741116` was queued with `steps=[]`, `runner_id=null`, and therefore was not passing evidence. This baseline commit requires a fresh exact-head run after the write.

## Feature specification and ubiquitous language

- **publication request**: caller intent containing only release identity and publisher target;
- **release authority evidence**: immutable release identity, source SHA-256, locale, approval state, and stable approval-evidence identity supplied by `ReleaseAuthorityPort`;
- **compatibility release identity**: exact immutable `content_release_id` and source SHA-256 that the target validator actually evaluated;
- **target compatibility evidence**: release-bound target, contract/version/standard, validation-evidence identity, and blockers supplied by `TargetCompatibilityPort`;
- **blocking feature**: authority-owned evidence that target transformation would lose semantics;
- **publication admission**: deterministic cross-binding of caller intent, immutable release authority, and release-bound target validation;
- **publication outcome**: opaque compatible/incompatible result preserving release and target authority traceability;
- **native-web publication receipt**: opaque downstream evidence after exact release/artifact/manifest hashing and validation-receipt canonicalization.

Admission invariants:

- caller cannot assert approval or omit blockers through `PublicationRequest`;
- release evidence must exist and exactly match requested release identity; authoritative approval must be true;
- source hash, locale, and approval-evidence identity are required; SHA-256 syntax is validated;
- target evidence must exist, match requested target, and identify the same exact `content_release_id` and `source_hash` as release authority;
- target-owned contract/version/standard/validation-evidence identities are required and target contract ownership is strict;
- blockers are validated, canonically sorted, and exact duplicates rejected;
- `PublicationOutcome`, `PublicationMetadata`, and `NativeWebPublicationReceipt` are externally read-only;
- admission does not prove byte equality; native finalization recomputes exact release SHA-256 and exact artifact/build-manifest SHA-256;
- byte-finalized receipt metadata preserves release-approval and target-validation evidence identities and canonicalizes the source digest from exact release bytes.

## DDD context map

- **Core — Content Authoring & Release:** mutable projects/revisions/review/approval and immutable release authority.
- **Supporting — Publication Admission & Projection:** authority-backed compatibility decision, exact release-to-validation binding, target transformation boundary, and byte provenance.
- **Supporting — Rights & Accessibility Evidence:** release and target gating evidence.
- **Generic — Artifact Storage / Delivery:** object storage/CDN/registry/telemetry/deployment behind ACLs.

`ReleaseAuthorityPort` and `TargetCompatibilityPort` are ACLs between owning bounded contexts and Publication Admission. `CompatibilityReleaseIdentity` prevents target-validation evidence from crossing immutable release boundaries. Native byte finalization is a distinct domain service after rendering and before storage. Production implementations must not reconstruct authority from mutable request fields or synthetic/demo data.

## Commercialization gaps

| Gap | Owner | Evidence | Action/state | Next verification |
| --- | --- | --- | --- | --- |
| Cached target compatibility evidence could authorize another release | Learning Content Studio | PR #6 review verified against source | **Repaired test-first** by `c8346a5f...` then `f00ebc15...`; review thread resolved | Fresh exact-head Quality + central review evidence |
| Caller assertions could forge trusted compatibility | Learning Content Studio | earlier PR #6 review | **Repaired test-first** with authority ports and intent-only request | Preserve under exact-head regression suite |
| Aggregate coverage could mask production gaps | Learning Content Studio | earlier PR #6 review | **Repaired** with per-`src/` line/branch enforcement | Exact-head coverage proves every production file 100% |
| Native finalizer diverged from current admission authority API | Learning Content Studio | PR #7 stale stack/source compared with PR #6 `c51d...` | **Repaired** on existing writer branch; source/tests adapted and child non-destructively merged onto current parent | Fresh exact-head fmt/clippy/tests/coverage/rustdoc + review |
| Parent foundation not protected-integrated | Learning Content Studio / governance | PR #1 `e7977e57...`; required workflows/approval incomplete | Open without bypass | Unchanged-head required workflows + independent approval + ordinary merge |
| Central workflow execution unavailable/queued | ContextualWisdomLab/.github / GitHub Actions | PR #6 and pre-baseline PR #7 jobs observed with no runner/steps | Fail closed; no local green substitution | Runner allocation + exact-head central/repository checks |
| Stacked central review | ContextualWisdomLab/.github | organization ruleset requires OpenCode and one approval on protected default integration | No self-approval or local substitute | Central stacked review and ordinary protected integration |
| Shared native xAPI 2.0 contract not released | `ContextualWisdomLab/learning-interoperability-contracts` | renderer explicitly dependency-gated | Open upstream | Release true owner before native renderer conformance claim |
| No native renderer/package generator | Learning Content Studio | finalizer consumes already-emitted bytes | Open | Deterministic renderer/manifest builder against released shared contract + byte-identical fixtures |
| No immutable persistence | Learning Content Studio | no schema/migration/repository | Open | 3NF append-only `content_release`/`publication_receipt` authority and audit transactions |
| No buyer-facing authoring UX | Learning Content Studio | no app/UI/Storybook/Figma evidence | Open | Review -> accessibility/rights -> approval -> release workflow with accessibility/error-recovery evidence |
| No operability/deployment baseline | Learning Content Studio | no service/container/runtime | Open | Add when remote persistence/publishing requires it; then compose/observability/recovery/k6 |
| No public product release | Learning Content Studio | no protected release/tag/package | Open | Protected integration, SBOM/provenance/reproducibility/API maturity |

## Persistence and security guardrails

Future authoritative relational objects use two-or-more-word `snake_case` names and 3NF. `content_release`, `release_component`, `release_asset`, `release_approval`, and `publication_receipt` are append-only where immutable. Item-level UPSERT is allowed only for explicitly mutable indexes with tested idempotency keys. Authority-port adapters are privileged security boundaries and require least privilege, auditability, retention, encryption, and recovery evidence appropriate to CSAP/SOC 2 design goals. Current kernels require no PII; fixtures/docs use no real persons/institutions.

## Verification matrix

- release-binding trust repair is test-first: `c8346a5fb1652c02a515b826f856a83e2072ae63` establishes release/source mismatch expectations before production `f00ebc1523a682d35307ea4b14593378d1b8d190`;
- native byte-finalization behavior retains its earlier test-first evidence (`2534b18c...`; digest-case regression `f0ac99ce...` before `084aa228...`) and is now adapted to authority ports without dropping edge cases;
- deterministic/hash-sensitive core logic remains Rust and SHA-256 uses pinned RustCrypto `sha2 = 0.11.0`;
- production consumes no synthetic demo data;
- public Rust APIs use `missing_docs = "deny"` plus rustdoc warnings-as-errors;
- CI requires rustfmt, Clippy `-D warnings`, all-target tests, and 100% per-production-file line/branch coverage with nonzero production branch evidence;
- central exact-head required workflows and independent review remain mandatory and cannot be replaced by predecessor-head or repository-local evidence;
- writer branches are re-fetched before mutation; stack repair uses ordinary ancestry and a non-force ref update.

## Next bounded commercialization slice

First re-establish exact-head checks/review for PRs #1, #6, and the restacked #7, integrating them only through ordinary ruleset-compliant merges. In parallel, the highest-value product gaps are (1) immutable append-only release/publication receipt persistence with explicit evidence/audit transaction identities and (2) releasing the shared native xAPI 2.0 contract in its true owner so Learning Content Studio can implement a conformant renderer without duplicating ecosystem authority.
