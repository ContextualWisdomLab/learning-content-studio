# Architecture

Learning Content Studio is the ContextualWisdomLab LCMS and authoring authority. It owns mutable authoring projects, reusable learning objects/assets, revisions, accessibility/localization/rights evidence, review/approval state, immutable content releases, publication admission, target publication provenance, and byte-finalized publication evidence. Enrollment, learner completion, xAPI statement truth, psychometric response/scoring truth, and payment truth remain outside this bounded context.

## Authoring and publication pipeline

Mutable authoring source -> review -> accessibility/rights validation -> approval -> immutable content release -> Publication Admission -> target renderer -> byte finalization -> Artifact Storage / Delivery.

## Domain-driven design

### Core subdomain: Content Authoring & Release

Future aggregate roots are `ContentProject` for mutable editorial transactions and append-only `ContentRelease` for immutable publication authority. Corrections create successor releases and preserve lineage; they do not mutate approved authority.

### Supporting subdomain: Publication Admission & Projection

Publication Admission separates caller intent from authority. `PublicationRequest` contains only `content_release_id` and `PublisherTarget`. It cannot carry approval, source hash, locale, target contract metadata, or blocking features.

Two security-sensitive authority ports are anti-corruption boundaries:

- `ReleaseAuthorityPort` supplies `ReleaseAuthorityEvidence`, including release identity, source SHA-256, locale, approval state, and stable approval-evidence identity;
- `TargetCompatibilityPort` supplies `TargetCompatibilityEvidence`, including a `CompatibilityReleaseIdentity` with the exact immutable `content_release_id` and `source_hash` validated, plus target, contract/version/standard, validation-evidence identity, and semantic blockers.

`evaluate_publication` cross-binds release evidence to caller intent and target evidence to both the requested target and exact immutable release identity/hash. Cached compatibility evidence from another release or source identity fails closed before contract/blocker admission. The service validates required identities and contract ownership, canonicalizes blockers, and creates opaque `PublicationOutcome` authority.

For native web, `finalize_native_web_publication` is a separate byte-owning domain service after rendering. It accepts only an opaque compatible native outcome, verifies the admitted source identity against exact immutable release bytes, hashes the exact emitted artifact and build-manifest bytes, canonicalizes validation-receipt identities, preserves release-approval and target-validation traceability, and returns an opaque `NativeWebPublicationReceipt`. It does not render, upload, persist, or claim xAPI conformance.

The publisher boundary remains an ACL: xAPI/cmi5/SCORM/Common Cartridge/QTI details may not leak into canonical authoring entities. Shared schemas are consumed from released `ContextualWisdomLab/learning-interoperability-contracts` contracts, never copied locally.

### Supporting subdomain: Rights & Accessibility Evidence

Rights, accessibility, and localization evidence gate immutable release approval and target compatibility. Target adapters reference approved evidence identities; they do not silently reinterpret or waive it.

### Generic subdomain: Artifact Storage / Delivery

Object storage, CDN, registry, telemetry, deployment, and recovery remain generic capabilities behind explicit ports. They may project immutable facts but never become authoring authority.

## Context map

```text
Authoring Surface (Inkspan ACL)
          |
          v
Content Authoring & Release ----> Rights & Accessibility Evidence
          |
          | immutable release authority
          v
ReleaseAuthorityPort -----> Publication Admission <----- TargetCompatibilityPort
                                  |                              |
                                  |                 exact release id + source hash
                                  |                              |
                                  +----> Native Web Renderer -> Byte Finalization
                                  |                                  |
                                  |                                  v
                                  |                     NativeWebPublicationReceipt
                                  +----> cmi5 Quartz adapter
                                  +----> later versioned adapters
                                                   |
                                                   v
                                          Artifact Storage / Delivery
```

Downstream LMS/LRS/assessment systems consume released projections/contracts and never write Studio authoring tables directly.

## Current domain objects and services

- `PublicationRequest` — caller intent only;
- `ReleaseAuthorityEvidence` — immutable-release/approval evidence value object;
- `CompatibilityReleaseIdentity` — exact immutable release/source identity validated by target authority;
- `TargetCompatibilityEvidence` — release-bound target-validation evidence value object;
- `BlockingFeature` — semantic incompatibility evidence;
- `PublicationMetadata` — validated release/target authority traceability;
- `PublicationOutcome` — opaque admission result;
- `NativeWebPublicationReceipt` — opaque exact-byte publication evidence;
- `evaluate_publication` — authority-backed admission domain service;
- `finalize_native_web_publication` — native byte-finalization domain service.

Future durable domain events (`content_release_approved`, `publication_admitted`, `publication_rejected`, `native_publication_finalized`) wait for persistence transaction semantics.

## Persistence direction

No database is introduced by this slice. Future authoritative relational objects use two-or-more-word `snake_case` names and 3NF. Mutable authoring transactions and immutable release/publication transactions stay separate. `content_release`, `release_component`, `release_asset`, `release_approval`, and `publication_receipt` are append-only where they represent immutable authority. Item-level UPSERT is reserved for explicitly mutable indexes with tested idempotency keys. Read/write separation or materialized views require measured contention/load evidence.
