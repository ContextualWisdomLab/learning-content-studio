# Publishing contract

Authoring sources are not delivery packages. Publication is an explicit, deterministic transformation from an approved immutable content release.

## Canonical input and determinism

The only valid publisher input is an approved immutable `content_release` plus an explicit publisher contract/version. Mutable authoring branches, review drafts, local wall-clock time, machine locale, environment ordering, and network-fetched content are not publisher inputs.

Deterministic normalization rules:

- text is UTF-8 with Unicode NFC normalization and LF line endings;
- structured map/object keys are serialized in deterministic lexical order;
- files are ordered by normalized UTF-8 path;
- locale is taken from the immutable release manifest and never inferred from the build host;
- timestamps come from immutable release metadata and are normalized to UTC; publication never injects the current time into hashed payload bytes;
- asset identifiers are derived from the content release identity plus normalized asset path/content identity, not from random or machine-local IDs;
- `source_hash` covers the canonical immutable release manifest and release-owned content bytes, not mutable authoring sources;
- `artifact_hash` covers the final emitted artifact bytes exactly;
- a publisher contract version change is explicit even when the target standard revision does not change.

Identical release bytes, publisher contract ID/version, and target parameters must produce identical artifact bytes and hashes or an identical incompatibility payload.

## Initial publisher targets

- `native_web_publisher`
- `cmi5_quartz_publisher`
- `scorm_1_2_publisher`
- `scorm_2004_publisher`
- `common_cartridge_publisher`
- `qti_3_0_reference_publisher`
- `static_html_publisher`

The QTI 3.0 publisher is reference-only in this baseline: it emits/binds approved QTI assessment references or metadata and does not claim complete course-to-QTI package conversion.

## Executable Publication Admission boundary

The first executable Rust slice does not yet build packages. It gates one approved immutable release before a target adapter is allowed to emit anything. `evaluate_publication` requires exact release identity, SHA-256 source identity, explicit target, publisher contract/version, target-standard revision and locale. A compatible result means only that target transformation may proceed; it is not an artifact, certification or interoperability-conformance claim.

The currently executable target/contract mapping is one-to-one:

| Publisher target | Required publisher contract | Runtime protocol boundary |
| --- | --- | --- |
| `native_web_publisher` | `native_cwl_xapi_2_0/v1` | native CWL / xAPI 2.0 |
| `cmi5_quartz_publisher` | `cmi5_quartz_xapi_1_0_3/v1` | cmi5 Quartz / xAPI 1.0.3 |

Selecting the other target's contract is a hard `ContractTargetMismatch`; no fallback, compatibility alias or cross-conversion is permitted. The other listed publisher targets remain documented future adapters and are not admitted by the current kernel.

## Machine-readable result contract

Every publication outcome uses the same target-revision field name, `standard_revision`, so consumers do not need outcome-specific field mapping.

A successful publication artifact records at least:

```text
content_release_id
source_hash
publisher_contract_id
publisher_version
standard_revision
locale_code
artifact_hash
build_manifest_hash
validation_receipt_ids
```

The current admission kernel emits the common authority fields plus `publication_status` and `blocking_features`; artifact-only hashes/receipt IDs are added by a later target adapter after actual bytes exist.

An incompatible publication returns a deterministic payload with this minimum shape:

```json
{
  "publication_status": "incompatible",
  "content_release_id": "release-reference",
  "publisher_contract_id": "publisher-contract-reference",
  "publisher_version": "1.0.0",
  "standard_revision": "target-revision",
  "source_hash": "sha256:...",
  "locale_code": "en-US",
  "blocking_features": [
    {
      "feature_code": "unsupported_feature",
      "source_component_reference": "component-reference",
      "reason_code": "semantic_loss_required"
    }
  ]
}
```

Blocking features are sorted deterministically by `feature_code`, then `source_component_reference`, then `reason_code`. Duplicate entries with all three keys equal are invalid rather than preserving input order.

## xAPI-version-specific publisher contracts

The native and cmi5 paths are separate contracts and may not share validation or silently cross-convert protocol behavior.

### `cmi5_quartz_xapi_1_0_3/v1`

- target: cmi5 Quartz, 1st Edition;
- runtime evidence mapping: xAPI 1.0.3 compatibility as required by Quartz;
- transformation: only Quartz-defined course/AU metadata and launch/runtime mappings are emitted;
- rejection: any feature requiring xAPI 2.0-only semantics, an unsupported cmi5 runtime behavior, or silent semantic loss returns `incompatible`;
- validation: Quartz/cmi5 and xAPI 1.0.3 compatibility rules only.

### `native_cwl_xapi_2_0/v1`

- target: CWL native learning activity;
- runtime evidence mapping: xAPI 2.0 through released learning-interoperability contracts;
- transformation: native release semantics map only through the xAPI 2.0 contract;
- rejection: no fallback to cmi5/xAPI 1.0.3 semantics is permitted;
- validation: xAPI 2.0/native contract rules only.

A publisher must never silently discard semantics. Unsupported source features produce the machine-readable incompatible result above. Published artifacts preserve provenance and validation evidence.
