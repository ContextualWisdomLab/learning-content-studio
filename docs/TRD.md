# Technical requirements — Publication Admission and Native-Web Finalization

## Scope

This TRD covers the first two executable Learning Content Studio publication boundaries. Publication Admission validates one approved immutable release against one target-specific interoperability contract and returns a deterministic compatible/incompatible result. Native-Web Finalization consumes only a trusted compatible native admission plus the exact canonical release bytes and already-emitted artifact/build-manifest bytes; it recomputes byte identities and returns a deterministic publication receipt. Target rendering, persistence, network services and UI remain outside this slice.

## Runtime and API

The implementation is a Rust library crate in `src/lib.rs` with `unsafe_code = "forbid"` and `missing_docs = "deny"`. SHA-256 uses the pinned RustCrypto `sha2 = 0.11.0` implementation rather than a handwritten cryptographic primitive.

Primary APIs:

```text
evaluate_publication(PublicationRequest)
  -> Result<PublicationOutcome, AdmissionError>

finalize_native_web_publication(
  PublicationOutcome,
  canonical_release_bytes,
  artifact_bytes,
  build_manifest_bytes,
  validation_receipt_ids
) -> Result<NativeWebPublicationReceipt, NativeWebPublicationError>
```

`PublicationRequest` contains exact immutable release identity, caller-supplied `sha256:` source identity, publisher target, target-owned contract ID, publisher version, target standard revision, locale, approval state and zero or more blocking features. Publication Admission validates SHA-256 syntax but cannot prove digest/content equality because it does not own release bytes.

A successful `PublicationOutcome` is a trusted result of this validation boundary. Its fields and `PublicationMetadata` fields are private outside the crate; callers receive read-only accessors. Downstream code cannot construct or mutate a value that impersonates a validated compatible/incompatible result. Admission preserves the exact validated source-hash spelling supplied by its caller for auditability.

`finalize_native_web_publication` is the first byte-owning trust boundary. It requires the exact canonical immutable release bytes, final emitted artifact bytes and build-manifest bytes. It recomputes SHA-256 over each byte set, compares the source digest to the admitted identity, and returns an opaque `NativeWebPublicationReceipt` carrying immutable authority metadata, `artifact_hash`, `build_manifest_hash` and canonically ordered validation receipt IDs. Receipt metadata records the recomputed lowercase `sha256:` source identity so equivalent upper/lowercase admission spellings cannot create different publication evidence. It does not render content and therefore does not claim that a complete native-web generator exists.

## Admission validation order

1. reject an unapproved release;
2. reject empty required identities, including `source_hash`;
3. reject a malformed SHA-256 source identity;
4. reject a publisher contract that belongs to another target;
5. reject incomplete blocking-feature identities;
6. sort blockers by `feature_code`, `source_component_reference`, `reason_code`;
7. reject exact duplicate blocker triples;
8. return compatible when no blockers remain, otherwise incompatible.

## Native-web finalization order

1. reject an incompatible trusted admission;
2. reject any trusted admission whose exact contract is not `native_cwl_xapi_2_0/v1`;
3. recompute SHA-256 from the exact canonical immutable release bytes and compare the digest to admitted `source_hash`;
4. reject a zero-byte emitted artifact;
5. reject a zero-byte build manifest;
6. reject empty validation-receipt identities;
7. sort validation-receipt identities lexically and reject exact duplicates;
8. compute exact SHA-256 identities for artifact and build-manifest bytes;
9. construct receipt metadata with the canonical lowercase recomputed source identity while retaining the original validated spelling in the admission outcome;
10. return an opaque deterministic native-web publication receipt.

Hexadecimal case is non-semantic for a valid admitted digest. Finalization compares the recomputed digest case-insensitively, admission preserves caller spelling, and publication receipt evidence uses the recomputed lowercase identity. No mutable authoring source, wall clock, environment locale, network fetch, random identifier or process ordering enters either decision.

## Deterministic evidence

`PublicationOutcome::canonical_json()` emits fixed top-level field order from a validation-only outcome whose blockers were canonically sorted before construction. Compatible outcomes cannot carry blockers; incompatible outcomes always carry at least one blocker.

`NativeWebPublicationReceipt::canonical_json()` emits fixed field order and canonically sorted validation receipt IDs. `artifact_hash` covers the final emitted artifact bytes exactly. `build_manifest_hash` covers the exact build-manifest bytes. Receipt `source_hash` is the lowercase SHA-256 identity recomputed from exact release bytes, so semantically identical uppercase/lowercase admitted hashes produce byte-identical receipt evidence.

## Failure model

Both boundaries fail closed. Cross-target contract selection, missing approval, malformed or missing source identity, missing required fields and duplicate blocker identities return typed admission errors. Native finalization additionally rejects incompatible outcomes, cmi5 outcomes, source-byte mismatch, empty artifact/manifest bytes, and invalid validation-receipt identities. Neither trusted outcome nor publication receipt has a public constructor.

## Test and coverage contract

`tests/publication_admission.rs` was committed before the first production kernel and subsequent behavior fixes remain test-first. `tests/native_web_publication.rs` was committed at `2534b18c3422e6f353cc74c3443f8834d4f58cd2` before the native finalization implementation. The digest-canonicalization regression identified by live review was committed first at `f0ac99ce16762638e76a5983892c5785db4f36b0`; production was repaired at `084aa22868f32157d13e63c99c9defbe6e9ac34a`. The suite covers:

- unapproved release rejection;
- native/cmi5 cross-target contract rejection;
- missing-prefix, wrong-length, non-hex and missing source-hash behavior;
- order-independent incompatibility output and duplicate blocker rejection;
- compatible authority preservation and JSON escaping;
- byte-identical native-web receipt reproduction independent of caller receipt-ID order;
- exact source-byte mismatch rejection;
- equivalent upper/lowercase admitted digest spellings producing identical canonical receipt evidence while admission preserves caller spelling;
- incompatible admission and cmi5 contract rejection at native finalization;
- empty emitted artifact/build-manifest rejection;
- empty and duplicate validation-receipt identity rejection.

Repository CI runs formatting, Clippy with warnings denied, all-target tests and rustdoc warnings-as-errors on `ubuntu-24.04`. It installs checksum-verified `cargo-llvm-cov` 0.9.0 through a commit-pinned `taiki-e/install-action`, installs exact `nightly-2026-08-30` with `llvm-tools-preview`, requires 100% line coverage, exports LLVM branch coverage, rejects missing/zero branch evidence, and requires `covered == count` for 100% branch coverage. Nightly is isolated to coverage measurement; production/lint/test semantics continue on the runner's stable Rust toolchain.

## Cryptographic implementation evidence

SHA-256 is the byte-identity primitive. NIST FIPS 180-4 remains the published Secure Hash Standard containing SHA-256 while NIST prepares a revision; current NIST CAVP material continues to list SHA-256 under FIPS 180-4. The implementation pins RustCrypto `sha2` 0.11.0, released 2026-03-25, and keeps all digest computation in safe Rust. Dependency provenance and vulnerability evidence remain subject to the protected Dependency Review/SAST/security gates; a missing dependency-graph comparison is never treated as clean.

## Security and operability

The crate has no network, filesystem, secret handling or unsafe Rust. The only new production dependency is the explicitly pinned RustCrypto SHA-2 implementation and its transitive digest/backend dependencies. Central Security/SAST/review workflows remain independent merge gates. A central Dependency Review 403 is intentionally not translated into success; repository security capability must be repaired before protected integration.

## Future boundaries

The next product slice is the actual deterministic native-web renderer/packager that produces `artifact_bytes` and `build_manifest_bytes` from a canonical immutable release and released native interoperability contract. It must feed those exact bytes into `finalize_native_web_publication`; no renderer may mint its own trusted receipt. Persistence follows with 3NF, two-or-more-word `snake_case` objects and append-only release/publication facts. A service/API boundary is introduced only when durable storage or remote publishing requires one; at that point asynchronous request handling, compose deployment and k6 latency/load evidence become mandatory.
