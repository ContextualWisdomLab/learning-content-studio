# Learning Content Studio

Learning Content Studio is the ContextualWisdomLab authoring and LCMS authority for evidence-bound learning content. It separates mutable authoring work from approved immutable releases so delivery artifacts can be reproduced without becoming a second source of truth.

## Product responsibility

The Studio owns authoring projects, reusable learning objects, assets, revisions, accessibility alternatives, localization, rights metadata, review and approval state, immutable content releases, and target-specific publication contracts. Enrollment, learner-completion policy, xAPI statement truth, psychometric response data, and commercial payment truth remain outside this bounded context.

The repository is currently pre-release. Documentation and active implementation work do not establish a hosted service, installable application, interoperability certification, or production publisher until protected integration and exact-head release evidence prove those states.

## Start here

- [Repository README](../README.md) — product value, boundaries, maturity, and publication model.
- [Architecture](ARCHITECTURE.md) — bounded context and system structure.
- [Publishing contract](PUBLISHING.md) — deterministic publication, compatibility, and evidence semantics.
- [Authoring and release ADR](adr/0001-authoring-release-boundary.md) — canonical ownership decision.
- [Standards traceability](doctoring/STANDARD_TRACEABILITY.md) — adopted standards, versions, and evidence expectations.
- [Changelog](../CHANGELOG.md) — repository-visible change history.
- [GitHub Releases](https://github.com/ContextualWisdomLab/learning-content-studio/releases) — versioned release artifacts when available.
- [Ask DeepWiki](https://deepwiki.com/ContextualWisdomLab/learning-content-studio) — repository-aware navigation and questions.

## Architecture and release boundary

Mutable drafts are never publication authority. Approved immutable releases are the source for deterministic target projections such as native web, cmi5, SCORM, Common Cartridge, bounded QTI reference publication, or static HTML. If a target cannot preserve required semantics, publication must return explicit incompatibility evidence instead of silently degrading meaning.

Shared learning interoperability contracts belong in `learning-interoperability-contracts`; Learning Management Platform owns learner/enrollment/completion state; Learning Record Store owns durable learning-record evidence. Integrations should use reviewed versioned contracts rather than cross-repository application-table reads.

## Security, accessibility, and verification

Accessibility, rights, provenance, tenant and authorization boundaries, dependency provenance, reproducible publication, and recovery evidence are part of release readiness. A standards citation or successful documentation check is not a conformance or certification claim.

## Publication status

This file is a GitHub Pages source prerequisite, not proof that Pages is live. Publication is complete only after this source reaches protected `develop`, the organization-owned repository metadata reconciler applies the reviewed Pages configuration, deployment succeeds, and the public HTTPS content is re-read successfully.
