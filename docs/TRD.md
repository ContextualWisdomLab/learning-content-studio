# Technical requirements — Publication Admission

## Scope

This TRD covers the first executable Learning Content Studio publication boundary. It validates one approved immutable release against one target-specific interoperability contract and returns a deterministic compatible/incompatible result. Package generation, persistence, network services and UI are outside this slice.

## Runtime and API

The implementation is a dependency-free Rust library crate in `src/lib.rs` with `unsafe_code = "forbid"` and `missing_docs = "deny"`.

Primary API:

```text
evaluate_publication(PublicationRequest)
  -> Result<PublicationOutcome, AdmissionError>
```

`PublicationRequest` contains exact immutable release identity, caller-supplied `sha256:` source identity, publisher target, target-owned contract ID, publisher version, target standard revision, locale, approval state and zero or more blocking features. Publication Admission validates SHA-256 syntax but does not possess release bytes and therefore cannot prove digest/content equality. The later byte-producing publisher must recompute the digest over the exact canonical immutable release bytes and compare it before artifact emission.

## Validation order

1. reject an unapproved release;
2. reject empty required identities;
3. reject a malformed SHA-256 source identity;
4. reject a publisher contract that belongs to another target;
5. reject incomplete blocking-feature identities;
6. sort blockers by `feature_code`, `source_component_reference`, `reason_code`;
7. reject exact duplicate blocker triples;
8. return compatible when no blockers remain, otherwise incompatible.

No mutable authoring source, wall clock, environment locale, network fetch, random identifier or process ordering is consulted.

## Deterministic result

`PublicationOutcome::canonical_json()` emits fixed top-level field order. Incompatible blockers are defensively cloned and canonically sorted during serialization, including when a caller directly constructs the public enum instead of obtaining it through `evaluate_publication`; caller vector order therefore cannot change canonical bytes. JSON strings are escaped deterministically, including control characters. This result is admission evidence only; `artifact_hash`, `build_manifest_hash` and validation-receipt IDs are added only after a target adapter creates actual artifact bytes.

## Failure model

Validation is fail-closed. Cross-target contract selection, missing approval, malformed source identity, missing required fields and duplicate blocker identities return typed errors rather than being silently normalized into a compatible decision.

## Test and coverage contract

`tests/publication_admission.rs` was committed before implementation and subsequent behavior fixes continue test-first. Coverage includes:

- unapproved release rejection;
- native/cmi5 cross-target contract rejection;
- missing-prefix, wrong-length and non-hex source-hash rejection;
- order-independent incompatibility output, including directly constructed outcomes;
- duplicate blocker rejection;
- compatible authority preservation and canonical JSON;
- every required request/blocker identity empty-field path;
- JSON quote, backslash, newline, carriage-return, tab, backspace, form-feed, generic control-character and ordinary Unicode escaping paths.

Repository CI runs formatting, Clippy with warnings denied, all-target tests and rustdoc warnings-as-errors on `ubuntu-24.04`. It installs checksum-verified `cargo-llvm-cov` 0.9.0 through a commit-pinned `taiki-e/install-action`, installs exact `nightly-2026-08-30` with `llvm-tools-preview`, requires 100% line coverage, exports LLVM branch coverage, rejects missing/zero branch evidence, and requires `covered == count` for 100% branch coverage. Nightly is isolated to coverage measurement; production/lint/test semantics continue on the runner's stable Rust toolchain.

## Security and operability

The crate has no external package dependency, network access, filesystem access, secret handling or unsafe Rust. Central Security/SAST/review workflows remain independent merge gates. A central Dependency Review 403 is intentionally not translated into success; repository security capability must be repaired before protected integration.

## Future boundaries

The next technical slice adds a native-web projection port that consumes only a compatible admission and canonical immutable release bytes. Its first invariant is recomputing `source_hash` from those exact bytes before any artifact is emitted. Persistence follows later with 3NF, two-or-more-word `snake_case` objects and append-only release/publication facts. A service/API boundary is introduced only when durable storage or remote publishing requires one; at that point asynchronous request handling, compose deployment and k6 latency/load evidence become mandatory.
