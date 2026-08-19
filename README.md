# Learning Content Studio

The LCMS and authoring authority for the CWL Learning Platform.

## Scope

The Studio owns mutable authoring projects, reusable learning objects, assets, revisions, accessibility alternatives, localization, rights metadata, review/approval state, and immutable content releases.

Published cmi5, SCORM, Common Cartridge, QTI reference, static HTML, and native web artifacts are deterministic projections of approved releases rather than the canonical source model. Unsupported target semantics are reported explicitly instead of being silently discarded.

Inkspan is the preferred reusable authoring surface; learning-specific semantics and publishing remain here.

## Branching

Product work targets `develop`; release promotion to `main` occurs only after exact-head review and required checks.

See `docs/ARCHITECTURE.md`, `docs/PUBLISHING.md`, and `docs/doctoring/STANDARD_TRACEABILITY.md`.
