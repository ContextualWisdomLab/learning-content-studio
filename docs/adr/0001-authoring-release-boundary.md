# ADR 0001: Authoring and release boundary

## Status

Accepted

Approved by: ContextualWisdomLab repository owner  
Approval date: 2026-08-19

## Decision

Mutable authoring state and immutable release artifacts are separate concerns. A published artifact is a deterministic projection of one approved content release through one versioned publisher contract.

## Consequences

SCORM, cmi5, Common Cartridge, QTI 3.0 reference artifacts, and native web output are publication targets rather than the canonical source model. The QTI target in this baseline is **reference-only**: it may emit or bind approved assessment references and metadata but does not claim to serialize an arbitrary Studio course as a complete QTI package. Full QTI package publication would require a separate publisher contract and conformance evidence. Unsupported semantics are reported as incompatibilities instead of being silently discarded.
