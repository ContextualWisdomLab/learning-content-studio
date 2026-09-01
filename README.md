# Learning Content Studio

[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/ContextualWisdomLab/learning-content-studio)

The LCMS and authoring authority for the CWL Learning Platform.

## Scope

The Studio owns mutable authoring projects, reusable learning objects, assets, revisions, accessibility alternatives, localization, rights metadata, review/approval state, and immutable content releases.

Published cmi5, SCORM, Common Cartridge, QTI 3.0 reference-only, static HTML, and native web artifacts are deterministic projections of approved releases rather than the canonical source model. The QTI 3.0 target in this baseline binds approved assessment references/metadata and does not claim arbitrary course-to-QTI package conversion. Unsupported target semantics are reported explicitly instead of being silently discarded.

Inkspan is the preferred reusable authoring surface; learning-specific semantics and publishing remain here.

## Branching and promotion

Product work targets `develop`. Promotion from `develop` to `main` requires an independent exact-head semantic review with no unresolved blocking thread, successful `Learning Content Studio Quality`, successful organization-required `Security Scan` and `SAST Semgrep`, and every additional check required by the live repository ruleset. Predecessor-head, queued, skipped-required, or stale-review evidence is not sufficient.

See `docs/ARCHITECTURE.md`, `docs/PUBLISHING.md`, and `docs/doctoring/STANDARD_TRACEABILITY.md`.
