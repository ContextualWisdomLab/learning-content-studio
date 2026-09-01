# Product and technical gap baseline

## Product responsibility

Learning Content Studio is the ContextualWisdomLab LCMS and authoring authority. It owns mutable authoring state, reusable learning objects and assets, review/approval, accessibility/localization/rights evidence, immutable `content_release` authority, and deterministic target-specific publication projections. Enrollment/completion, xAPI record truth, psychometric response/scoring truth, and payment truth remain outside this bounded context.

## Exact-head evidence contract

This baseline applies to the exact Git commit that contains it. Every merge decision must bind to the live PR head SHA, not a predecessor description or merge candidate. The active stack is `agent/bootstrap-learning-content-studio` -> `agent/publishing-admission-kernel` -> `agent/native-web-artifact-projection`; GitHub PR/branch metadata is the canonical live SHA source.

Evidence established during the active commercialization iteration:

- repository was re-fetched live and is public, organization-owned, and `fork=false`;
- parent PR #1 exact head `7b8e0472451ab095301b7a5e40b1f99cd8af584b` remains open, Ready and mechanically mergeable; all observed inline review threads are resolved and CodeRabbit/Devin commit statuses are successful, but no qualifying `APPROVED` review is present;
- parent PR #1 fresh exact-head Learning Content Studio Quality run `33550893313`, SAST Semgrep run `33550893331`, and Security Scan run `33550893471` are queued at the latest read; Quality job `99999764275` has no executed steps yet, so predecessor successes and the historical Dependency Review HTTP 403 incident are not promoted to current-head evidence;
- the live organization ruleset still requires the central workflow set, one approving review, and resolved review threads on the protected default branch; no bypass, self-approval or ruleset weakening is accepted;
- PR #6 exact head `68bd5778851890247e76cc0abaa408742da3716c` is open, Ready and mechanically mergeable; repository quality run `33543213021` completed successfully on that exact head and the latest Devin review reported zero new potential issues;
- PR #6 review defects were repaired test-first: canonical blocker ordering, coverage enforcement, forged trusted outcomes, and missing-vs-malformed `source_hash` classification. The latest source-hash regression was committed first at `cbecbcf98a7426051c343c956346906221d4df1e`, production at `2cb6e32f548f960349902e2a0d18a2c6e78854af`, and its live review thread was resolved only after exact-head verification;
- PR #6 retains informational review notes that admission validates digest syntax rather than release bytes and preserves hexadecimal case as input identity. Those are intentional admission semantics rather than open product defects; the downstream byte-owning boundary addresses byte equality without rewriting admission authority;
- native-web byte-finalization regression/edge-case tests were committed first at `2534b18c3422e6f353cc74c3443f8834d4f58cd2`; production then pinned RustCrypto SHA-256 and implemented the byte evidence boundary;
- live PR #7 review found that preserving admission digest casing inside the final receipt made equivalent source identities produce different evidence. The regression was committed first at `f0ac99ce16762638e76a5983892c5785db4f36b0`; production was repaired at `084aa22868f32157d13e63c99c9defbe6e9ac34a` by retaining exact caller spelling in admission while canonicalizing receipt `source_hash` to the lowercase recomputed identity;
- PR #7 head `70c6bb9c7e8af3bcb6a8b70ca59d09bbd478e2e0` failed both repository `validate` jobs (`99980586697`, `99980569965`) deterministically at `cargo fmt --check`: the new `uppercase_hash` expression in `tests/native_web_publication.rs` was not rustfmt-canonical. Checkout and product-contract validation succeeded before the formatter gate. Commit `3242cf0b7064bdcc399e48e40b9bc055f180d973` applied only rustfmt-equivalent line wrapping;
- successor head `4b9c0a162f5cb136f69d3201373a752e010205a1` proved the formatter repair but exposed a second deterministic repository-owned defect in both `validate` jobs (`99990711884`, `99990694209`). Both failed at Clippy with Rust E0277 because `Sha256::digest(bytes)` no longer implements `LowerHex` through the pinned `sha2` 0.11 / `digest` 0.11 output. Existing known-hash tests are the test-first contract; commit `e0edf3b86c264993644566fbad0681b5796e4642` fixed only digest-byte encoding by serializing each computed byte to lowercase hexadecimal without weakening any gate;
- pre-refresh PR #7 exact head `b550cf4a5fdf86f72a903500c1cdcab416abfcc4` is open, Ready and mechanically mergeable. Its current review threads contain only informational observations: external validation-receipt identifiers are intentionally opaque/exact aside from non-empty validation, empty canonical release bytes remain a valid SHA-256 input, and source-hash case normalization occurs only after byte verification. No unresolved thread currently identifies a demonstrated behavior defect;
- pre-refresh PR #7 Quality run `33554112273` / job `100010610968` is queued with no executed steps at the latest read. This documentation refresh moves the writer-branch head, so that predecessor queued run is non-transferable and fresh exact-head repository evidence is required;
- central required-workflow rules target only `~DEFAULT_BRANCH`, so stacked PRs rely on the documented central stacked-review path until they are retargeted onto protected `develop`; no repository-local approval substitute is accepted.

## Current feature specification

### Publication Admission bounded context

The first executable commercialization slice is the Rust fail-closed admission kernel in `src/lib.rs`.

Ubiquitous language:

- **content release**: approved immutable source authority presented to a publisher;
- **publisher target**: a delivery family with its own interoperability contract;
- **publisher contract**: version-specific transformation/validation boundary owned by one target;
- **blocking feature**: source semantic that cannot be preserved by the selected target;
- **publication admission**: deterministic decision allowing target transformation or returning incompatibility evidence;
- **trusted publication outcome**: an immutable result that can only be created after the admission invariants succeed;
- **native-web publication receipt**: immutable byte-bound evidence created only after compatible native admission and exact-byte SHA-256 verification;
- **artifact hash**: SHA-256 identity of the final emitted artifact bytes;
- **build manifest hash**: SHA-256 identity of the exact build-manifest bytes used for that artifact.

Admission aggregate/invariants:

- `PublicationRequest` is the transaction boundary for one immutable release and one selected target;
- release approval is mandatory before admission;
- `source_hash` is a required non-empty identity field and must then satisfy the explicit SHA-256 syntax contract;
- missing/whitespace-only `source_hash` returns `EmptyRequiredField("source_hash")`, while a non-empty malformed digest returns `InvalidSourceHash`;
- `native_web_publisher` admits only `native_cwl_xapi_2_0/v1`;
- `cmi5_quartz_publisher` admits only `cmi5_quartz_xapi_1_0_3/v1`;
- cross-target contract selection fails closed;
- blocking features sort by `feature_code`, `source_component_reference`, then `reason_code` before trusted outcome construction;
- duplicate blocking-feature triples are invalid at admission;
- a compatible trusted outcome has zero blockers and an incompatible trusted outcome has at least one blocker;
- `PublicationOutcome` and `PublicationMetadata` have no external write/constructor surface; validated status, authority metadata and blockers are exposed read-only;
- admission metadata preserves the exact validated caller-supplied SHA-256 spelling for auditability;
- canonical JSON output has stable field and array order from the already validated outcome.

Native-web finalization invariants:

- `finalize_native_web_publication` accepts only a trusted compatible outcome owned by `native_cwl_xapi_2_0/v1`;
- it recomputes SHA-256 from the exact canonical immutable release bytes and fails closed on mismatch with admitted `source_hash`;
- upper/lower hexadecimal case is treated as the same digest during comparison, while final receipt `source_hash` is the canonical lowercase identity recomputed from exact bytes; equivalent admission spellings therefore produce identical receipt evidence;
- zero-byte emitted artifacts and zero-byte build manifests are invalid;
- validation-receipt identities are opaque external identities: they must be non-empty when present, are preserved exactly (including case and non-empty surrounding whitespace), sort lexically, and may not contain exact duplicates;
- empty canonical release bytes remain valid input when their admitted SHA-256 identity matches; the finalizer forbids empty emitted artifact and build-manifest bytes, not a mathematically valid empty release byte string;
- `artifact_hash` and `build_manifest_hash` are computed inside the trusted finalizer from exact bytes rather than supplied by callers;
- `NativeWebPublicationReceipt` has no public constructor or mutable authority surface;
- `NativeWebPublicationReceipt::canonical_json()` emits fixed field order and canonical receipt-ID order;
- this is byte-finalization/provenance evidence only: rendering/transformation is still an explicit open gap and a receipt does not imply xAPI 2.0 conformance or certification.

Domain events are not persisted in these slices. A future persistence boundary may emit `content_release_approved`, `publication_admitted`, `publication_rejected`, and `native_publication_finalized` only after durable transaction semantics are defined.

## DDD context map

- **Core subdomain — Content Authoring & Release:** mutable authoring projects, revisions, review/approval and immutable releases.
- **Supporting subdomain — Publication Admission & Projection:** deterministic compatibility decision, target-specific transformation and byte-bound finalization evidence.
- **Supporting subdomain — Rights & Accessibility Evidence:** release-gating evidence referenced by immutable release identity and publication receipt.
- **Generic subdomain — Artifact Storage / Delivery:** object storage, CDN, registry and deployment mechanisms; these must remain behind ports/ACLs and must not become authoring truth.

The publisher boundary is an anti-corruption layer: cmi5/xAPI 1.0.3, native xAPI 2.0, SCORM, Common Cartridge and bounded QTI 3.0 semantics may not leak into the canonical authoring model. A future native renderer consumes a released learning-interoperability contract; it must not duplicate that repository's shared schema authority.

## Commercialization gaps

| Gap | Owner | Live evidence | Action/state | Next verification |
| --- | --- | --- | --- | --- |
| No executable release/publisher boundary | Learning Content Studio | PR #1 explicitly states no executable publisher exists | **Partially repaired:** PR #6 supplies trusted admission; native stack adds byte-level finalization evidence | Exact-head fmt/clippy/test/rustdoc/coverage and independent review on both stack levels |
| Admission source identity was syntax-only | Learning Content Studio | live Devin info note correctly observed admission cannot prove digest/content equality | **Addressed at correct downstream boundary:** native finalizer recomputes exact release SHA-256 before receipt creation | Exact-head tests prove mismatch rejection and case-equivalent digest comparison |
| Receipt evidence depended on source-hash casing | Learning Content Studio | Devin PR #7 review showed uppercase/lowercase equivalent admitted digests yielded unequal receipt metadata/JSON | **Repaired test-first:** regression `f0ac99ce...`; production `084aa228...`; admission preserves caller spelling but final receipt uses recomputed lowercase identity | Exact-head tests/coverage and independent reviewer confirmation |
| PR #7 exact-head formatting gate failed | Learning Content Studio | `validate` jobs `99980586697` and `99980569965` both stop at the same `cargo fmt --check` diff in `tests/native_web_publication.rs`; checkout/product-contract preflight succeeded | **Repaired at causal source:** `3242cf0b...` applies rustfmt-canonical wrapping only | Succeeded past `cargo fmt --check` on successor exact head; predecessor failure is not transferable |
| PR #7 `sha2` 0.11 digest formatting did not compile | Learning Content Studio | exact-head jobs `99990711884` and `99990694209` both reach Clippy then fail E0277 because `Sha256::digest(bytes)` returns the `digest` 0.11 / `hybrid-array` output without `LowerHex` | **Repaired at causal source:** `e0edf3b...` keeps pinned RustCrypto 0.11 and explicitly serializes computed digest bytes to lowercase hex; existing known-hash fixtures are the prior failing tests | Fresh final-head fmt/clippy/test/100%-coverage/rustdoc must complete terminal-success |
| Canonical JSON depended on caller blocker order | Learning Content Studio | Devin PR #6 review showed equivalent blocker evidence could serialize differently | **Repaired test-first** | Exact-head tests and independent reviewer confirmation |
| Trusted admission result could be forged by downstream caller | Learning Content Studio | Devin PR #6 review showed public outcome variants/public metadata fields bypassed checks | **Repaired test-first:** regression `96d97f4d...`; production `26f530ca...` | Exact-head tests, rustdoc and independent reviewer confirmation |
| Missing source identity was misclassified as malformed | Learning Content Studio | Devin PR #6 review showed empty `source_hash` returned `InvalidSourceHash` | **Repaired test-first:** regression `cbecbcf9...`; production `2cb6e32f...` | Exact-head tests and reviewer confirmation |
| 100% coverage mandate was unenforced | Learning Content Studio | Devin PR #6 review found tests ran without line/branch measurement | **Repaired:** pinned `cargo-llvm-cov` + exact nightly branch instrumentation | Every new stack head must pass line/branch gate |
| Repository quality runner admission | Learning Content Studio / GitHub Actions control plane | PR #6 exact-head run `33543213021` succeeded on explicit `ubuntu-24.04`; PR #7 pre-refresh run `33554112273` is currently queued with job steps absent | **Not claimed green on the new head:** explicit selector is retained and no no-op push/gate weakening is used to manufacture runner admission | Fresh exact-head job must acquire a runner, verify immutable checkout, and complete all repository gates |
| Dependency Review evidence on protected parent | GitHub repository security configuration / organization control plane | a predecessor Security Scan failed closed on GitHub HTTP 403; fresh PR #1 exact-head Security Scan `33550893471` is queued | **Historical incident retained, current state pending:** do not promote the predecessor 403 or any predecessor green result to the new head | Let the fresh exact-head required workflow execute; repair the true security/control-plane cause only if the current run reproduces it |
| Stacked OpenCode evidence not yet materialized | ContextualWisdomLab/.github control plane | default-branch ruleset does not inject workflows into stack bases; central sweep is fallback | **Existing causal path retained; no unsafe local substitute** | Central sweep produces current-head review receipt or stack is retargeted after parent integration |
| No immutable release persistence | Learning Content Studio | no schema/migration/repository implementation exists | Open | Define 3NF `content_release`, `release_component`, `release_asset`, `release_approval`, `publication_receipt`; immutable facts append-only; explicit item-level UPSERT only for mutable authoring indexes |
| No actual target renderer/package generator | Learning Content Studio | byte finalizer receives already-emitted bytes and explicitly performs no transformation | **Narrowed:** exact byte-provenance trust boundary now exists | Implement deterministic native renderer/manifest builder against a released shared xAPI 2.0 contract, then feed exact bytes into finalizer and prove byte-identical fixtures |
| Shared native xAPI 2.0 contract not yet released | learning-interoperability-contracts | its protected `develop` remains bootstrap-only; open PR #1 states executable xAPI 2.0/profile conformance is still issue #3 | Dependency-gated | Repair/release shared xAPI 2.0 contract in true owner before claiming native renderer conformance |
| No buyer-facing authoring workflow | Learning Content Studio | no application/UI/Storybook/Figma evidence exists | Open | Implement review -> accessibility/rights gate -> approval -> release flow; verify keyboard/touch/screen-reader/error recovery before GA |
| No operability/deployment baseline | Learning Content Studio | no service/container/runtime exists | Open | Add service only when publisher/storage workflow requires it; then compose deployment, observability, recovery and k6 evidence |
| No release/package evidence | Learning Content Studio | no product release/tag/package exists | Open | Publish only after protected integration, SBOM/provenance, reproducible artifacts and public API maturity |
| CEFR assessment-content vertical | Learning Content Studio + learning-interoperability-contracts | Issues #4/#5 require a released shared contract | Dependency-gated | Consume released `cwl_cefr_language_assessment/v1`; do not duplicate shared schema or protected descriptor prose |

## Persistence and data design guardrails

No relational schema is introduced by the current kernels. When persistence arrives, authoritative relational objects use two-or-more-word `snake_case` names, remain in 3NF, and separate mutable authoring facts from immutable release/publication facts. Generic one-word persistence object names such as a table named `id` are forbidden. Immutable release rows are append-only; publication receipts reference exact release and contract identities. Item-level UPSERT is permitted only where mutable index semantics are explicit and tested; it must never overwrite immutable release/publication authority. Hot publication/read paths should be separated from authoring writes only when contention evidence appears rather than pre-emptively denormalizing truth.

## Security, compliance and privacy

The current kernels process identifiers, compatibility evidence and byte buffers only; they do not require PII. SHA-256 byte identity follows NIST FIPS 180-4 as the currently published Secure Hash Standard containing SHA-256, while NIST has announced a future revision. The implementation uses pinned RustCrypto `sha2` 0.11.0 rather than handwritten cryptography. Security remains fail-closed when dependency evidence is unavailable. Future author/reviewer identity and rights/consent data must use least-privilege access, auditability, retention policy and encryption appropriate to SOC 2/CSAP objectives. Real people or institutions must not appear in fixtures.

## Verification matrix

- behavior changes remain test-first: initial admission tests preceded production implementation; canonical-order repair, trusted-outcome opacity and missing-vs-malformed source identity were test-first; native byte-finalization tests were committed at `2534b18c...` before implementation; receipt source-identity canonicalization regression was committed at `f0ac99ce...` before production `084aa228...`; the same pre-implementation known-hash fixtures are the failing contract that caught the `sha2` 0.11 digest-serialization compile defect before `e0edf3b...` repaired it;
- Rust owns deterministic admission, SHA-256 byte identity and publication-receipt logic;
- no synthetic demo data is consumed by production;
- public Rust API documentation is mandatory through `missing_docs = "deny"` and rustdoc warnings-as-errors;
- CI requires formatting, Clippy with warnings denied, all-target tests, 100% line coverage, nonzero 100% branch coverage, and rustdoc;
- coverage tooling is versioned and checksum-verified; branch instrumentation uses exact `nightly-2026-08-30`;
- the new SHA-256 dependency is pinned to RustCrypto `sha2` 0.11.0 and must pass protected dependency/security evidence before integration;
- exact-head central security/SAST/review evidence remains mandatory for protected integration and is never replaced by repository-local green checks.

## Next bounded commercialization slice

Implement the actual deterministic native-web renderer/manifest builder only after the shared xAPI 2.0/native interoperability contract has a released immutable authority. The renderer must consume canonical immutable release data, produce byte-identical artifact and build-manifest bytes from identical inputs, reject semantic loss, preserve rights/accessibility evidence references, and pass those exact bytes through `finalize_native_web_publication`. Do not add cmi5 transformation to the native path; cmi5 remains a separate version-specific adapter.