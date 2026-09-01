# ADR 0001: Authoring and release boundary

## Status

Accepted

Approved by: ContextualWisdomLab repository owner  
Approval date: 2026-08-19

## Decision

Mutable authoring state and immutable release authority are separate concerns. A published artifact is a deterministic projection of one approved `content_release` through one versioned publisher target contract.

Publication Admission must not treat caller assertions as release or compatibility authority. Caller intent contains only the requested release identity and publisher target. The release-owning bounded context supplies approval/source/locale evidence through `ReleaseAuthorityPort`; the target-validation bounded context supplies contract/version/standard/blocker evidence through `TargetCompatibilityPort`. `evaluate_publication` cross-binds both evidence objects to caller intent before minting an opaque admission outcome.

The authority ports are anti-corruption layers and privileged trust boundaries. Production implementations must derive evidence from authoritative immutable release and target-validation sources, never from request booleans, omittable request blocker arrays, mutable authoring rows, or synthetic/demo data.

## Consequences

SCORM, cmi5, Common Cartridge, QTI 3.0 reference artifacts, and native web output remain publication targets rather than canonical source models. The QTI target in this baseline is reference-only unless a distinct complete-package contract and conformance evidence are established. Unsupported semantics are reported as target-authority incompatibilities instead of being silently discarded.

Admission proves that the configured authority ports were consulted and their evidence cross-bound; it does not prove artifact bytes or interoperability conformance. Byte-producing publishers must recompute source/artifact/manifests from exact bytes, and native xAPI conformance remains gated on released shared interoperability contracts.