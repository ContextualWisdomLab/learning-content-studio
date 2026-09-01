# Product and technical gap baseline

## Product responsibility

Learning Content Studio is the ContextualWisdomLab LCMS and authoring authority. It owns mutable authoring state, reusable learning objects and assets, review/approval, accessibility/localization/rights evidence, immutable `content_release` authority, and deterministic target-specific publication projections. Enrollment/completion, xAPI record truth, psychometric response/scoring truth, and payment truth remain outside this bounded context.

## Exact-head evidence contract

This baseline applies to the exact Git commit that contains it. Every verification record must bind to the live PR head SHA, not a predecessor description or merge candidate. The current implementation branch is `agent/publishing-admission-kernel`, stacked on `agent/bootstrap-learning-content-studio`; GitHub PR metadata is the canonical live SHA source.

Evidence already established on the parent foundation head:

- repository is public, organization-owned, and `fork=false`;
- PR #1 is open, Ready, and mechanically mergeable;
- all inline review threads observed on PR #1 are resolved;
- CodeRabbit and Devin status contexts are successful;
- Semgrep, Trivy, Scorecard, and OSV exact-head jobs completed successfully;
- central Dependency Review failed closed because GitHub returned HTTP 403 for the exact base/head dependency-graph comparison;
- the repository quality job remained queued on `ubuntu-latest`, so this implementation slice pins `ubuntu-24.04` for its own executable quality lane rather than weakening the security gate.

## Current feature specification

### Publication Admission bounded context

The first executable commercialization slice is a Rust fail-closed admission kernel in `src/lib.rs`.

Ubiquitous language:

- **content release**: approved immutable source authority presented to a publisher;
- **publisher target**: a delivery family with its own interoperability contract;
- **publisher contract**: version-specific transformation/validation boundary owned by one target;
- **blocking feature**: source semantic that cannot be preserved by the selected target;
- **publication admission**: deterministic decision allowing target transformation or returning incompatibility evidence.

Aggregate/invariants:

- `PublicationRequest` is the transaction boundary for one immutable release and one selected target;
- release approval is mandatory before admission;
- `source_hash` must be an explicit SHA-256 identity;
- `native_web_publisher` admits only `native_cwl_xapi_2_0/v1`;
- `cmi5_quartz_publisher` admits only `cmi5_quartz_xapi_1_0_3/v1`;
- cross-target contract selection fails closed;
- blocking features sort by `feature_code`, `source_component_reference`, then `reason_code`;
- duplicate blocking-feature triples are invalid;
- compatible and incompatible outcomes bind the same release/contract/version/standard/locale authority;
- canonical JSON output has stable field and array order.

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
| No executable release/publisher boundary | Learning Content Studio | PR #1 explicitly states no executable publisher exists | **In progress:** Rust Publication Admission kernel and tests on `agent/publishing-admission-kernel` | Exact-head fmt/clippy/test/rustdoc plus independent review |
| Repository quality lane can wait indefinitely on `ubuntu-latest` | Learning Content Studio | PR #1 quality run remained queued while central jobs obtained `ubuntu-24.04` runners | **Addressed in implementation branch:** pin repository quality to `ubuntu-24.04` | Exact-head workflow job obtains a runner and executes all steps |
| Dependency Review cannot obtain evidence | GitHub repository security configuration / organization control plane | central Security Scan exact-head probe returned HTTP 403 | **Blocked, fail-closed:** do not bypass or convert to green skip | Enable/repair dependency-graph capability or token entitlement, then rerun exact-head Security Scan |
| No immutable release persistence | Learning Content Studio | no schema/migration/repository implementation exists | Open | Define 3NF `content_release`, `release_component`, `release_asset`, `release_approval`, `publication_receipt`; item-level UPSERT only for mutable authoring indexes, never immutable releases |
| No target artifact generator | Learning Content Studio | admission slice deliberately emits no package/artifact | Open | Implement native-web projection first with byte-identical fixture proof; then cmi5 Quartz behind its distinct contract |
| No buyer-facing authoring workflow | Learning Content Studio | no application/UI/Storybook/Figma evidence exists | Open | Implement review→accessibility/rights gate→approval→release flow; verify keyboard/touch/screen-reader/error recovery before GA |
| No operability/deployment baseline | Learning Content Studio | no service/container/runtime exists | Open | Add service only when publisher/storage workflow requires it; then compose deployment, observability, recovery and k6 evidence |
| No release/package evidence | Learning Content Studio | no product release/tag/package exists | Open | Publish only after protected integration, SBOM/provenance, reproducible artifacts and public API maturity |
| CEFR assessment-content vertical | Learning Content Studio + learning-interoperability-contracts | Issues #4/#5 require a released shared contract | Dependency-gated | Consume released `cwl_cefr_language_assessment/v1`; do not duplicate shared schema or protected descriptor prose |

## Persistence and data design guardrails

No relational schema is introduced by the admission kernel. When persistence arrives, authoritative relational objects use two-or-more-word `snake_case` names, remain in 3NF, and separate mutable authoring facts from immutable release/publication facts. Generic one-word persistence object names such as a table named `id` are forbidden. Immutable release rows are append-only; publication receipts reference exact release and contract identities. Hot publication/read paths should be separated from authoring writes when contention evidence appears rather than pre-emptively denormalizing truth.

## Security, compliance and privacy

The current kernel processes identifiers and compatibility evidence only; it does not require PII. Security remains fail-closed when dependency evidence is unavailable. Future author/reviewer identity and rights/consent data must use least-privilege access, auditability, retention policy and encryption appropriate to SOC 2/CSAP objectives. Real people or institutions must not appear in fixtures.

## Verification matrix

- behavior change is test-first: `tests/publication_admission.rs` was committed before production implementation;
- Rust owns the deterministic publication-admission logic;
- no synthetic demo data is consumed by production;
- public Rust API documentation is mandatory through `missing_docs = "deny"` and rustdoc warnings-as-errors;
- CI requires formatting, Clippy with warnings denied, all-target tests, and rustdoc;
- exact-head central security/SAST/review evidence remains mandatory and is never replaced by repository-local green checks.

## Next bounded commercialization slice

After the admission PR is protected-integrated, implement a native-web artifact projection from an approved immutable release, with a canonical byte manifest, artifact SHA-256 computation, rights/accessibility validation receipts, byte-identical fixture reproduction, and explicit incompatibility output. Do not add cmi5 transformation to the native path; cmi5 remains a separate version-specific adapter.
