# Learning Content Studio

[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/ContextualWisdomLab/learning-content-studio)

**Author, govern, approve, and publish reusable learning content from one evidence-bound source of truth.**

Learning Content Studio is the LCMS and authoring authority for the ContextualWisdomLab learning ecosystem. It owns the content lifecycle before delivery: mutable authoring projects, reusable learning objects, assets, revisions, accessibility alternatives, localization, rights metadata, review and approval state, and immutable content releases.

Its central promise is simple: **author once against an explicit content model, approve an immutable release, then publish deterministic target artifacts without turning those artifacts into a second source of truth.**

## Why it exists

Learning content is more than a page or a package. Teams need to know which revision was approved, whether accessibility and rights requirements were checked, which source evidence produced an artifact, and whether a target format preserved the intended semantics.

| Need | What Learning Content Studio owns |
| --- | --- |
| Reusable authoring | Structured learning objects, components, assets, revisions, and reusable authoring projects |
| Governed release | Review, approval, accessibility and rights validation before immutable content release |
| Accessible delivery | Accessibility alternatives, localization, WCAG 2.2 Level AA scope, and ATAG-aligned authoring requirements |
| Deterministic publication | Reproducible publication from immutable releases with explicit publisher contracts and hashes |
| Multiple delivery targets | Native web, cmi5, SCORM, Common Cartridge, static HTML, and bounded QTI 3.0 reference publication |
| Traceability | Source/release identity, publisher version, artifact hashes, validation receipts, and standards mapping |

## Product boundary

Learning Content Studio owns **content authoring and publication authority**. It does not own enrollment, learner completion, xAPI statement truth, psychometric response data, or commercial payment truth.

```text
mutable authoring project
          │
          ▼
 review → accessibility validation → rights validation → approval
          │
          ▼
 immutable content release
          │
          ├── native web artifact
          ├── cmi5 artifact
          ├── SCORM artifact
          ├── Common Cartridge artifact
          ├── QTI 3.0 reference artifact
          └── static HTML artifact
```

Published packages are projections of an approved immutable release. They are not canonical authoring state and must not silently discard unsupported semantics.

Inkspan is the preferred reusable authoring surface. Generic editing and composition capabilities may be reused there; learning-specific content semantics, release authority, standards mapping, rights, accessibility, and publisher behavior remain owned here.

## Current state

This repository is currently an **architecture and publication-contract foundation**, not an executable LCMS release. It establishes the authoring/release boundary, deterministic publishing rules, standards traceability, repository quality gate, and contribution policy.

There is currently no installable application, package, hosted service, or production publishing runtime to claim. The next executable slices must implement the structured content model, validation boundaries, immutable release storage, publisher adapters, and compatibility tests before this README can offer a real installation or runtime quickstart.

## Publication model

A publisher accepts only an approved immutable `content_release` plus an explicit publisher contract/version. Mutable drafts, host locale, wall-clock time, environment ordering, and network-fetched content are excluded from deterministic publisher input.

The initial target families are:

- **Native web** for the CWL-native learning experience;
- **cmi5 Quartz** with its bounded xAPI 1.0.3 interoperability contract;
- **SCORM 1.2 and SCORM 2004**;
- **Common Cartridge**;
- **QTI 3.0 reference-only publication**, which binds approved assessment references or metadata and does not claim complete course-to-QTI package conversion;
- **Static HTML**.

If a target cannot preserve required semantics, publication returns a deterministic machine-readable `incompatible` result instead of silently dropping meaning. See [`docs/PUBLISHING.md`](docs/PUBLISHING.md) for the canonical normalization, hashing, result, and target-contract rules.

## Architecture at a glance

The initial bounded modules are:

- structured content model;
- reusable component registry;
- asset library;
- revision control;
- accessibility validator;
- localization workflow;
- rights management;
- release pipeline;
- publisher adapters.

The repository architecture is intentionally standalone at the authoring/release boundary. Delivery systems consume released artifacts and contracts; they do not become writers of Studio authoring state.

## Standards and accessibility

The foundation tracks the standards that materially shape authoring and publication behavior rather than presenting standards names as certification claims. Current traceability includes WCAG 2.2 Level AA scope for authoring UI and generated content, ATAG 2.0 authoring principles, the adopted ISO/IEC 24751 accessibility parts, ISO/IEC 19788-1:2024 as a metadata framework, and target-specific learning interoperability standards.

See [`docs/doctoring/STANDARD_TRACEABILITY.md`](docs/doctoring/STANDARD_TRACEABILITY.md) for the exact editions, adopted scope, and requirement-to-product mapping.

## Verification

The repository-owned quality workflow validates the required architecture, publishing, governance, and standards-traceability files and rejects unresolved bootstrap placeholders. This foundation does not treat documentation checks as runtime conformance evidence; executable publishers will require their own compatibility, determinism, accessibility, security, and regression tests.

## Documentation map

| Goal | Start here |
| --- | --- |
| Product and bounded-context architecture | [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) |
| Deterministic publishing contract | [`docs/PUBLISHING.md`](docs/PUBLISHING.md) |
| Authoring/release decision | [`docs/adr/0001-authoring-release-boundary.md`](docs/adr/0001-authoring-release-boundary.md) |
| Standards and accessibility traceability | [`docs/doctoring/STANDARD_TRACEABILITY.md`](docs/doctoring/STANDARD_TRACEABILITY.md) |
| Change history | [`CHANGELOG.md`](CHANGELOG.md) |

## Contributing

Product work targets `develop`. Promotion from `develop` to `main` requires the repository's live protected-branch governance, exact-head checks, applicable security gates, review requirements, and zero valid unresolved blocking threads. Repository-specific contributor rules are in [`AGENTS.md`](AGENTS.md).

Keep authoring truth, immutable release authority, and target publication contracts separate when adding a feature. A target adapter must fail explicitly when it cannot preserve required source semantics.

## License

Learning Content Studio source and documentation are licensed under the [Apache License 2.0](LICENSE). Third-party specifications, cited standards, dependencies, and future imported assets retain their own licenses and must remain compatible with the repository's commercial-use and attribution policy.
