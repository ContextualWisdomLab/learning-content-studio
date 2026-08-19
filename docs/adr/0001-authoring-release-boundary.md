# ADR 0001: Authoring and release boundary

## Status

Proposed

## Decision

Mutable authoring state and immutable release artifacts are separate concerns. A published artifact is a deterministic projection of one approved content release through one versioned publisher.

## Consequences

SCORM, cmi5, Common Cartridge, QTI references, and native web output are publication targets rather than the canonical source model. Unsupported semantics are reported as incompatibilities instead of being silently discarded.
