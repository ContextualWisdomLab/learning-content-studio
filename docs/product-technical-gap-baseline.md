# Product and technical gap baseline

## Product responsibility

Learning Content Studio is the ContextualWisdomLab LCMS and authoring authority. It owns mutable authoring state, reusable learning objects and assets, review/approval, accessibility/localization/rights evidence, immutable `content_release` authority, and deterministic target-specific publication projections. Enrollment/completion, xAPI record truth, psychometric response/scoring truth, and payment truth remain outside this bounded context.

## Exact-head evidence contract

This baseline applies to the exact Git commit that contains it. Every merge decision must bind to the live PR head SHA, not a predecessor description or merge candidate. The current implementation branch is `agent/publishing-admission-kernel`, stacked on `agent/bootstrap-learning-content-studio`; GitHub PR metadata is the canonical live SHA source.

Evidence established during the active commercialization iteration:

- repository is public, organization-owned, and `fork=false`;
- parent PR #1 is open, Ready, mechanically mergeable, and all observed inline review threads are resolved;
- CodeRabbit and Devin status contexts succeeded on the parent exact head;
- Semgrep completed successfully on the parent exact head; central Security Scan remains failed because Dependency Review received GitHub HTTP 403 for the exact base/head dependency-graph comparison;
- parent repository quality remains queued on `ubuntu-latest`; this implementation pins `ubuntu-24.04`, and an earlier implementation head completed formatting, Clippy, tests, and rustdoc on an assigned Ubuntu 24.04 runner;
- Devin review identified a canonical-serialization ordering defect and missing coverage gate. The ordering regression was committed before its production repair and the quality lane now enforces checksum-verified 100% line and branch coverage;
- a later Devin review found that public `PublicationOutcome` variants and public `PublicationMetadata` fields allowed downstream callers to manufacture apparently trusted compatible/incompatible results without running admission validation. Regression expectations were committed first at `96d97f4dc22fcff30e6a887256711b57f1fa9db1`; production was repaired at `26f530ca9763b92703b80aa2049d04fe9d74e1c9`; the trusted outcome and metadata are now externally read-only and created only by `evaluate_publication`;
- current-head review then found that an empty `source_hash` was being collapsed into `InvalidSourceHash`, making missing input indistinguishable from a non-empty malformed digest. The regression was committed first at `cbecbcf98a7426051c343c956346906221d4df1e`; production was repaired at `2cb6e32f548f960349902e2a0d18a2c6e78854af` by applying the required-field check before SHA-256 syntax validation;
- central required-workflow rules on this repository target only `~DEFAULT_BRANCH`, so stacked PR #6 does not receive injected OpenCode/Noema/Security workflows. The central `.github` control plane documents a rotating `org-queue-sweep` review path for stacked work; no local approval substitute is accepted.

## Current feature specification

### Publication Admission bounded context

The first executable commercialization slice is a Rust fail-closed admission kernel in `src/lib.rs`.

Ubiquitous language:

- **content release**: approved immutable source authority presented to a publisher;
- **publisher target**: a delivery family with its own interoperability contract;
- **publisher contract**: version-specific transformation/validation boundary owned by one target;
- **blocking feature**: source semantic that cannot be preserved by the selected target;
- **publication admission**: deterministic decision allowing target transformation or returning incompatibility evidence;
- **trusted publication outcome**: an immutable result that can only be created after the admission invariants succeed.

Aggregate/invariants:

- `PublicationRequest` is the transaction boundary for one immutable release and one selected target;
- release approval is mandatory before admission;
- `source_hash` is a required non-empty identity field and must then satisfy the explicit SHA-256 syntax contract; admission validates syntax only because it does not own release bytes, and the later byte-producing publisher must recompute/compare the digest before emission;
- missing/whitespace-only `source_hash` returns `EmptyRequiredField("source_hash")`, while a non-empty malformed digest returns `InvalidSourceHash`;
- `native_web_publisher` admits only `native_cwl_xapi_2_0/v1`;
- `cmi5_quartz_publisher` admits only `cmi5_quartz_xapi_1_0_3/v1`;
- cross-target contract selection fails closed;
- blocking features sort by `feature_code`, `source_component_reference`, then `reason_code` before trusted outcome construction;
- duplicate blocking-feature triples are invalid at admission;
- a compatible trusted outcome has zero blockers and an incompatible trusted outcome has at least one blocker;
- `PublicationOutcome` and `PublicationMetadata` have no external write/constructor surface; validated status, authority metadata and blockers are exposed read-only;
- compatible and incompatible outcomes bind the same release/contract/version/standard/locale authority;
- canonical JSON output has stable field and array order from the already validated outcome.

Domain events are not persisted in this first slice. A future persistence boundary may emit `content_release_approved`, `publication_admitted`, and `publication_rejected` events only after durable transaction semantics are defined.

## DDD context map

- **Core subdomain — Content Authoring & Release:** mutable authoring projects, revisions, review/approval and immutable releases.
- **Supporting subdomain — Publication Admission & Projection:** deterministic compatibility decision and target-specific projection.
- **Supporting subdomain — Rights & Accessibility Evidence:** release-gating evidence referenced by immutable release identity.
- **Generic subdomain — Artifact Storage / Delivery:** object storage, CDN, registry and deployment mechanisms; these must remain behind ports/ACLs and must not become authoring truth.

The publisher boundary is an anti-corruption layer: cmi5/xAPI 1.0.3, native xAPI 2.0, SCORM, Common Cartridge and bounded QTI 3.0 semantics may not leak into the canonical authoring model.

## Commercialization gaps

| Gap | Owner | Live evidence | Action/state | Next verification |
| --- | --- | --- | --- | --- |
| No executable release/publisher boundary | Learning Content Studio | PR #1 explicitly states no executable publisher exists | **In progress:** Rust Publication Admission kernel and tests in PR #6 | Exact-head fmt/clippy/test/rustdoc/coverage plus independent review |
| Canonical JSON depended on caller blocker order | Learning Content Studio | Devin PR #6 review showed equivalent blocker evidence could serialize differently | **Repaired test-first** | Exact-head tests and independent reviewer confirmation |
| Trusted admission result could be forged by downstream caller | Learning Content Studio | Devin PR #6 review showed public outcome variants/public metadata fields bypassed approval/hash/contract checks | **Repaired test-first:** regression expectations `96d97f4d...`; production `26f530ca...`; outcomes/metadata are opaque with read-only accessors | Exact-head tests, rustdoc and independent reviewer confirmation |
| Missing source identity was misclassified as malformed | Learning Content Studio | Devin PR #6 review showed empty `source_hash` returned `InvalidSourceHash` | **Repaired test-first:** regression `cbecbcf9...`; production `2cb6e32f...`; missing vs malformed machine-readable errors are distinct | Exact-head tests and reviewer confirmation |
| 100% coverage mandate was unenforced | Learning Content Studio | Devin PR #6 review found tests ran without line/branch measurement | **Repaired:** pinned `cargo-llvm-cov` 0.9.0 + exact nightly branch instrumentation; line and branch evidence fail closed below 100% | Exact-head coverage workflow succeeds with nonzero branch count |
| Repository quality lane can wait indefinitely on `ubuntu-latest` | Learning Content Studio | PR #1 quality run remained queued while central jobs obtained `ubuntu-24.04` runners | **Repaired in PR #6:** pin repository quality to `ubuntu-24.04` | Latest exact head obtains a runner and executes expanded coverage lane |
| Dependency Review cannot obtain evidence | GitHub repository security configuration / organization control plane | central Security Scan parent exact-head probe returned HTTP 403 | **Blocked, fail-closed:** do not bypass or convert to green skip | Enable/repair dependency-graph capability or token entitlement, then rerun exact-head Security Scan after retargeting to protected default |
| Stacked OpenCode evidence not yet materialized | ContextualWisdomLab/.github control plane | default-branch ruleset does not inject workflows into stack bases; documented rotating `org-queue-sweep` is the intended fallback | **Existing causal repair path verified; no unsafe local substitute added** | Central sweep dispatches PR #6 exact head and a current-head OpenCode receipt appears |
| No immutable release persistence | Learning Content Studio | no schema/migration/repository implementation exists | Open | Define 3NF `content_release`, `release_component`, `release_asset`, `release_approval`, `publication_receipt`; item-level UPSERT only for mutable authoring indexes, never immutable releases |
| No target artifact generator | Learning Content Studio | admission slice deliberately emits no package/artifact | Open | Implement native-web projection first with recomputed source digest and byte-identical fixture proof; then cmi5 Quartz behind its distinct contract |
| No buyer-facing authoring workflow | Learning Content Studio | no application/UI/Storybook/Figma evidence exists | Open | Implement review→accessibility/rights gate→approval→release flow; verify keyboard/touch/screen-reader/error recovery before GA |
| No operability/deployment baseline | Learning Content Studio | no service/container/runtime exists | Open | Add service only when publisher/storage workflow requires it; then compose deployment, observability, recovery and k6 evidence |
| No release/package evidence | Learning Content Studio | no product release/tag/package exists | Open | Publish only after protected integration, SBOM/provenance, reproducible artifacts and public API maturity |
| CEFR assessment-content vertical | Learning Content Studio + learning-interoperability-contracts | Issues #4/#5 require a released shared contract | Dependency-gated | Consume released `cwl_cefr_language_assessment/v1`; do not duplicate shared schema or protected descriptor prose |

## Persistence and data design guardrails

No relational schema is introduced by the admission kernel. When persistence arrives, authoritative relational objects use two-or-more-word `snake_case` names, remain in 3NF, and separate mutable authoring facts from immutable release/publication facts. Generic one-word persistence object names such as a table named `id` are forbidden. Immutable release rows are append-only; publication receipts reference exact release and contract identities. Hot publication/read paths should be separated from authoring writes when contention evidence appears rather than pre-emptively denormalizing truth.

## Security, compliance and privacy

The current kernel processes identifiers and compatibility evidence only; it does not require PII. Security remains fail-closed when dependency evidence is unavailable. Future author/reviewer identity and rights/consent data must use least-privilege access, auditability, retention policy and encryption appropriate to SOC 2/CSAP objectives. Real people or institutions must not appear in fixtures.

## Verification matrix

- behavior changes are test-first: initial admission tests preceded production implementation; canonical-order repair was test-first; trusted-outcome opacity was captured by integration expectations before its production repair; source-hash missing-vs-malformed classification was captured by regression commit `cbecbcf9...` before production commit `2cb6e32f...`;
- Rust owns the deterministic publication-admission logic;
- no synthetic demo data is consumed by production;
- public Rust API documentation is mandatory through `missing_docs = "deny"` and rustdoc warnings-as-errors;
- CI requires formatting, Clippy with warnings denied, all-target tests, 100% line coverage, nonzero 100% branch coverage, and rustdoc;
- coverage tooling is versioned (`cargo-llvm-cov` 0.9.0) and installed by a commit-pinned checksum-verifying action; branch instrumentation uses exact `nightly-2026-08-30`;
- exact-head central security/SAST/review evidence remains mandatory for protected integration and is never replaced by repository-local green checks.

## Next bounded commercialization slice

After the admission PR is protected-integrated, implement a native-web artifact projection from an approved immutable release, with recomputation/comparison of the canonical release SHA-256 identity, a canonical byte manifest, artifact SHA-256 computation, rights/accessibility validation receipts, byte-identical fixture reproduction, and explicit incompatibility output. Do not add cmi5 transformation to the native path; cmi5 remains a separate version-specific adapter.
