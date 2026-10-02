# Product and technical gap baseline

## Product responsibility

Learning Content Studio is the ContextualWisdomLab LCMS and authoring authority. It owns mutable authoring state, review/approval, accessibility/localization/rights evidence, immutable `content_release` authority, publication admission, target projection, byte-finalized publication evidence, and publication provenance. Enrollment/completion, xAPI record truth, psychometric response/scoring truth, and payment truth remain outside this bounded context.

## Exact-head evidence contract

This baseline applies to the exact Git commit that contains it. GitHub PR/branch metadata is canonical for live SHA identity; predecessor workflow/review evidence never transfers to a successor head.

Live commercialization evidence at the last pre-write re-fetch:

- repository is public, organization-owned, and `fork=false`;
- the active organization ruleset `CWL Central required workflows` applies to the default branch and requires one approving review, resolved review threads, and central close-empty/OpenCode/merge-scheduler/security/Strix/SAST/Noema workflows; no self-approval, admin bypass, or ruleset weakening is used by this work;
- PR #1 is open/Draft/mechanically mergeable at `e7977e5736b425e6221481934b25811ab27d7557`; that head is a tree-identical evidence refresh with no source delta over its predecessor. Exact-head SAST Semgrep `33569943093` and Learning Content Studio Quality `33569943228` passed, while Security Scan `33569943004` failed closed because GitHub Dependency Review returned HTTP 403;
- the canonical stack order is `PR #1 -> PR #6 -> PR #7`; each child remains Draft/Proposed until its parent is ordinarily integrated and its own unchanged exact-head gates finish;
- PR #6 is the first executable Publication Admission kernel and remains open/Draft. Ordinary merge `fa418842f7840f9bd69e0101c5b9677a6f38f22d` integrates current PR #1 exact head `e7977e5736b425e6221481934b25811ab27d7557`; current repair head `1201ab98a6a6676823f865a24d7535be072d4822` restores the complete regression file after a bounded tree-write error and preserves two current-Clippy assertion fixes. Its authority and release-binding defects remain repaired, and its quality workflow asserts the exact PR head, enforces statement/region plus line/branch coverage, avoids the `cargo-llvm-cov` output-path collision, and uses committed locked resolution;
- release-binding regression commit `c8346a5fb1652c02a515b826f856a83e2072ae63` preceded production repair `f00ebc1523a682d35307ea4b14593378d1b8d190`; mismatched release identity or source hash therefore fails closed;
- PR #7 remains open/Draft. Ordinary merge `5eedb2afad255232e75cd0be57fa4276fd830756` integrates repaired PR #6 exact head `1201ab98a6a6676823f865a24d7535be072d4822` while preserving all native-byte and durable-receipt delta without force-push or destructive rebase;
- fresh exact-head repository and central checks are mandatory after every child commit; predecessor workflow/review evidence never transfers.

## Feature specification and ubiquitous language

- **publication request**: caller intent containing only release identity and publisher target;
- **release authority evidence**: immutable release identity, source SHA-256, locale, approval state, and stable approval-evidence identity supplied by `ReleaseAuthorityPort`;
- **compatibility release identity**: exact immutable `content_release_id` and source SHA-256 that the target validator actually evaluated;
- **target compatibility evidence**: release-bound target, contract/version/standard, validation-evidence identity, and blockers supplied by `TargetCompatibilityPort`;
- **blocking feature**: authority-owned evidence that target transformation would lose semantics;
- **publication admission**: deterministic cross-binding of caller intent, immutable release authority, and release-bound target validation;
- **publication outcome**: opaque compatible/incompatible result preserving release and target authority traceability;
- **native-web publication receipt**: opaque downstream evidence after exact release/artifact/manifest hashing and validation-receipt canonicalization.
- **publication receipt store**: append-only repository for exact canonical receipt evidence keyed by release and publisher contract; it is not release approval, artifact storage, or relational audit authority.

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
| Synthetic merge checkout could mislabel quality evidence as exact-head | Learning Content Studio | PR #6 review verified against workflow | **Repaired test-first** with explicit PR-head checkout and SHA assertion | Exact-head workflow run proves checkout identity |
| Line/branch coverage could miss uncovered statement regions | Learning Content Studio | PR #6 review verified against workflow | **Repaired test-first** with per-`src/` statement/region, line, and branch enforcement | Exact-head coverage proves every production file 100% |
| Public blocker fields exposed an invariant-bypass surface | Learning Content Studio | current PR #6 API review verified against `BlockingFeature` | **Repaired test-first** with private identity fields and documented read-only accessors | Exact-head API tests preserve returned values while constructors remain the mutation boundary |
| Unlinked production source could be absent from LLVM coverage evidence | Learning Content Studio | current PR #6 review verified that the gate enumerated only LLVM records | **Repaired test-first** by comparing the repository `src/**/*.rs` inventory with reported production paths and failing on any missing file | Exact-head workflow contract and coverage run prove inventory completeness before per-file thresholds |
| Coverage report path collided with `cargo-llvm-cov` cleanup | Learning Content Studio | local execution failed after all tests with `No such file or directory` for `target/llvm-cov/coverage.json` | **Repaired test-first** by moving output to `target/coverage.json` | Exact-head workflow produces and parses the report |
| Publication request ownership failed denied Clippy lint | Learning Content Studio | local `cargo clippy --all-targets -- -D warnings` at `src/lib.rs:452` | **Repaired** by consuming command fields internally while preserving the value-taking public API | Exact-head Clippy and regression suite |
| Cargo resolution was not immutable | Learning Content Studio | `Cargo.lock` absent from PR #6 and downstream PR #7 | **Repaired test-first at the stack foundation** by committing the lockfile and requiring `--locked`; PR #7 restack updates it for native dependencies | Exact-head locked build on each stack layer |
| Native finalizer diverged from current admission authority API | Learning Content Studio | earlier PR #7 stale stack/source | **Repaired** on the existing writer branch; source/tests adapted and child non-destructively merged onto current parent | Fresh exact-head fmt/clippy/tests/coverage/rustdoc + review |
| PR #6 and PR #7 ancestry lagged their current parent heads | Learning Content Studio | live PR base/head comparison on 2026-10-02 | **Repaired without force** by ordinary parent merges `fa418842...` and `5eedb2af...`; the interrupted bounded tree write is preserved in history and corrected by complete-file child `1201ab98...` | Require new exact-head hosted gates on both Draft children; prior runs do not transfer |
| Native digest projection failed denied Clippy lint | Learning Content Studio | restacked local `cargo clippy --locked --all-targets -- -D warnings` at `src/lib.rs:703` | **Repaired** with direct reference iteration and no behavior change | Exact-head Clippy and byte-vector regressions |
| Parent foundation not protected-integrated | Learning Content Studio / governance | PR #1 `e7977e57...`; required workflows/approval incomplete | Open without bypass | Unchanged-head required workflows + independent approval + ordinary merge |
| Dependency Review availability/configuration | ContextualWisdomLab/.github / GitHub configuration | exact-head PR #1 dependency compare HTTP 403; canonical `.github#810` | Fail closed | Authorized control-plane repair + exact-head canary |
| Central workflow execution unavailable/queued | ContextualWisdomLab/.github / GitHub Actions | predecessor PR #6/#7 jobs observed with no runner/steps | Fail closed; no local green substitution | Runner allocation + exact-head central/repository checks |
| Stacked central review | ContextualWisdomLab/.github | organization ruleset requires OpenCode and one approval on protected default integration | No self-approval or local substitute | Central stacked review and ordinary protected integration |
| Native byte finalization integration | Learning Content Studio | PR #7 owns exact-byte receipt evidence | Restacked on current admission/quality foundation | Exact-head validation and review |
| Receipt evidence vanished on process restart | Learning Content Studio | PR #7 file repository tests and exact local branch-coverage report | **Repaired in bounded scope** with atomic no-overwrite install, parent/store fsync, exact replay, conflict/corruption failure, and orphan recovery; ADR 0003 remains Proposed until integration | Fresh exact-head fmt/Clippy/tests/100% coverage/rustdoc + protected review |
| Shared native xAPI 2.0 contract not released | `ContextualWisdomLab/learning-interoperability-contracts` | renderer explicitly dependency-gated | Open upstream | Release true owner before native renderer conformance claim |
| No native renderer/package generator | Learning Content Studio | finalizer consumes already-emitted bytes | Open | Deterministic renderer/manifest builder against released shared contract + byte-identical fixtures |
| No authoritative immutable release/audit persistence | Learning Content Studio | bounded filesystem repository stores receipt evidence only; no schema/migration/transaction ledger | Open | 3NF append-only `content_release`/`publication_receipt` authority, audit transactions, retention/resource limits, recovery evidence |
| No buyer-facing authoring UX | Learning Content Studio | no app/UI/Storybook/Figma evidence | Open | Review -> accessibility/rights -> approval -> release workflow with accessibility/error-recovery evidence |
| No operability/deployment baseline | Learning Content Studio | no service/container/runtime | Open | Add when remote persistence/publishing requires it; then compose/observability/recovery/k6 |
| No public product release | Learning Content Studio | no protected release/tag/package | Open | Protected integration, SBOM/provenance/reproducibility/API maturity |

## Persistence and security guardrails

The current file repository stores exact canonical receipt evidence only. It requires an operator-provisioned, same-filesystem, privately writable directory; digest checks detect corruption but not a privileged writer that can recompute them. Future authoritative relational objects use two-or-more-word `snake_case` names and 3NF. `content_release`, `release_component`, `release_asset`, `release_approval`, and `publication_receipt` are append-only where immutable. Item-level UPSERT is allowed only for explicitly mutable indexes with tested idempotency keys. Authority-port adapters are privileged security boundaries and require least privilege, auditability, retention, encryption, capacity limits, and recovery evidence appropriate to CSAP/SOC 2 design goals. Current kernels require no PII; fixtures/docs use no real persons/institutions.

## Verification matrix

- release-binding trust repair is test-first: `c8346a5fb1652c02a515b826f856a83e2072ae63` establishes release/source mismatch expectations before production `f00ebc1523a682d35307ea4b14593378d1b8d190`;
- native byte-finalization behavior retains its earlier test-first evidence (`2534b18c...`; digest-case regression `f0ac99ce...` before `084aa228...`) and is now adapted to authority ports without dropping edge cases;
- receipt persistence was test-first for write/reopen/read exactness, idempotent and conflicting replay, and interrupted-write recovery; review-driven regressions cover parent/replay directory fsync, PID-reuse orphan collision, corruption, missing/unreadable storage, and deterministic commit/sync failure;
- deterministic/hash-sensitive core logic remains Rust and SHA-256 uses pinned RustCrypto `sha2 = 0.11.0`;
- production consumes no synthetic demo data;
- public Rust APIs use `missing_docs = "deny"` plus rustdoc warnings-as-errors;
- CI requires an asserted exact-head checkout, rustfmt, locked Clippy `-D warnings`, locked all-target tests, complete `src/**/*.rs` presence in LLVM evidence, and 100% per-production-file statement/region, line, and branch coverage from the committed resolution with nonzero production region and branch evidence;
- central exact-head required workflows, Security/SAST, and independent review remain mandatory and cannot be replaced by predecessor-head or repository-local evidence;
- local verification after the stack repair passed rustfmt, 46 Rust tests, locked Clippy with warnings denied, locked rustdoc, and `git diff --check`; hosted exact-head evidence is still required;
- writer branches are re-fetched before mutation; stack repair uses ordinary ancestry and a non-force ref update without destructive rebase.

## Next bounded commercialization slice

First re-establish exact-head checks/review for PRs #1, #6, and #7, integrating them only through ordinary ruleset-compliant merges. In parallel, the highest-value product gaps are (1) authoritative 3NF append-only release/publication audit transactions with retention/resource/recovery evidence and (2) releasing the shared native xAPI 2.0 contract in its true owner so Learning Content Studio can implement a conformant renderer without duplicating ecosystem authority.
