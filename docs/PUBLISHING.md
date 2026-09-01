# Publishing contract

Authoring sources are not delivery packages. Publication is an explicit deterministic projection from one approved immutable content release through one versioned publisher target contract.

## Authority before transformation

Caller intent is not publication authority. `PublicationRequest` selects only a `content_release_id` and `PublisherTarget`. Publication Admission obtains all trust-bearing facts through explicit authority ports:

- `ReleaseAuthorityPort` returns immutable release identity, SHA-256 source identity, locale, approval state, and approval-evidence identity;
- `TargetCompatibilityPort` returns a `CompatibilityReleaseIdentity` for the exact immutable `content_release_id` and `source_hash` it validated, plus selected target, target-owned contract/version/standard, target-validation evidence identity, and semantic blockers.

A production authority port is an anti-corruption adapter to the real owning bounded context. It must not reconstruct evidence from request booleans, request blocker lists, synthetic demo state, or mutable authoring rows. `evaluate_publication` cross-binds release authority to caller release intent and target evidence to both target intent and the exact immutable release identity/hash. Cached target evidence for another release fails closed.

## Canonical input and determinism

Mutable authoring branches, review drafts, local wall-clock time, machine locale, environment ordering, and network-fetched content are not publisher inputs.

Deterministic normalization direction:

- text is UTF-8 with Unicode NFC normalization and LF line endings;
- structured keys serialize in deterministic lexical order;
- files order by normalized UTF-8 path;
- locale comes from immutable release authority and is never inferred from the build host;
- timestamps come from immutable release metadata and never inject wall-clock time into hashed payload bytes;
- asset identifiers derive from release identity plus normalized path/content identity, not random/machine-local IDs;
- `source_hash` covers canonical immutable release manifest/content bytes and is supplied by release authority at admission, cross-bound to target validation, then recomputed by the byte-owning finalizer;
- `artifact_hash` covers final emitted artifact bytes exactly;
- `build_manifest_hash` covers the exact build-manifest bytes used for that artifact;
- validation receipt identities are exact external evidence references, lexically ordered and exact-duplicate rejected;
- publisher contract/version/standard come from the target-validation authority.

## Executable target boundaries

| Publisher target | Required publisher contract | Runtime protocol boundary |
| --- | --- | --- |
| `native_web_publisher` | `native_cwl_xapi_2_0/v1` | native CWL / xAPI 2.0 |
| `cmi5_quartz_publisher` | `cmi5_quartz_xapi_1_0_3/v1` | cmi5 Quartz / xAPI 1.0.3 |

Target authority evidence naming another target, another target's contract, another release identity, or another source hash fails closed. No compatibility alias, fallback, stale-evidence replay, or silent cross-conversion is permitted.

Future adapters remain `scorm_1_2_publisher`, `scorm_2004_publisher`, `common_cartridge_publisher`, `qti_3_0_reference_publisher`, and `static_html_publisher`. QTI remains reference-only unless a separate complete package contract and conformance evidence are established.

## Machine-readable admission evidence

A successful admission outcome preserves at least:

```text
publication_status
content_release_id
release_approval_evidence_id
publisher_contract_id
publisher_version
standard_revision
target_validation_evidence_id
source_hash
locale_code
blocking_features
```

Blocking features are canonicalized by `feature_code`, then `source_component_reference`, then `reason_code`; exact duplicate triples are invalid. A compatible outcome is possible only when the target authority returns zero blockers. Callers cannot omit blockers through `PublicationRequest` because no such request field exists.

Admission proves validation against authority evidence and the exact release identity target validation evaluated. It does not prove artifact bytes or interoperability conformance.

## Native-web byte finalization

`finalize_native_web_publication` consumes only an opaque compatible `native_cwl_xapi_2_0/v1` admission plus exact canonical release bytes, already-emitted artifact bytes, exact build-manifest bytes, and validation-receipt identities. It:

1. rejects incompatible or non-native admission;
2. recomputes release SHA-256 and compares it case-insensitively with the authority-backed admitted digest;
3. rejects zero-byte artifact/build-manifest payloads;
4. rejects empty validation-receipt identities, sorts exact identities lexically, and rejects exact duplicates;
5. computes `artifact_hash` and `build_manifest_hash` from exact bytes;
6. canonicalizes receipt `source_hash` to the lowercase recomputed digest while preserving release-approval and target-validation evidence identities;
7. returns an opaque `NativeWebPublicationReceipt`.

A native receipt records:

```text
publication_status = published
content_release_id
release_approval_evidence_id
publisher_contract_id
publisher_version
standard_revision
target_validation_evidence_id
source_hash
locale_code
artifact_hash
build_manifest_hash
validation_receipt_ids
```

The finalizer does **not** render content, validate an xAPI profile, persist/upload artifacts, or certify interoperability. A complete native renderer remains gated on a released shared xAPI 2.0 contract from `ContextualWisdomLab/learning-interoperability-contracts`.

## xAPI-version-specific contracts

`cmi5_quartz_xapi_1_0_3/v1` remains a cmi5 Quartz / xAPI 1.0.3 boundary. `native_cwl_xapi_2_0/v1` remains a native xAPI 2.0 boundary and must consume the released shared contract before any renderer claims conformance. Unsupported semantics produce deterministic incompatibility evidence rather than silent semantic loss.
