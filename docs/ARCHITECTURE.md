# Architecture

The Learning Content Studio is the LCMS and authoring authority. It owns mutable authoring projects, reusable learning objects, revisions, accessibility variants, localization, rights metadata, review/approval state, immutable content releases, target publication decisions and publication provenance receipts.

It does not own enrollment, learner completion, xAPI statement truth, psychometric response data, or commercial payment truth.

## Authoring and publication pipeline

Mutable authoring source -> review -> accessibility validation -> rights validation -> approval -> immutable content release -> Publication Admission -> target-specific renderer -> byte-level publication finalization -> Artifact Storage / Delivery.

## Domain-driven design

### Core subdomain: Content Authoring & Release

The core bounded context owns the lifecycle from mutable authoring work through immutable approval. Its future aggregate roots are `ContentProject` for mutable editorial transactions and `ContentRelease` for immutable publication authority. A release never mutates after approval; corrections create a successor release and preserve lineage.

### Supporting subdomain: Publication Admission & Projection

`src/lib.rs` implements the executable trust boundaries that precede durable storage or delivery.

`PublicationRequest` is a minimal transaction boundary for one approved immutable release and one publisher target. `evaluate_publication` enforces release approval, SHA-256 source identity syntax, target-specific contract ownership, required identity fields, canonical blocker ordering, and duplicate-blocker rejection. `PublicationOutcome` is opaque outside the crate and can only be minted by that validated path.

Compatible admission is permission to proceed to a target adapter, not evidence that an artifact was built or certified. Incompatible admission is deterministic machine-readable evidence that publication would lose required semantics.

`finalize_native_web_publication` is the next transaction boundary after a native renderer has produced bytes. It accepts only a trusted compatible `native_cwl_xapi_2_0/v1` outcome, recomputes SHA-256 from the exact canonical immutable release bytes, rejects source mismatch, hashes the exact emitted artifact and build manifest, canonicalizes validation-receipt identities, and returns an opaque `NativeWebPublicationReceipt`. The receipt is byte-provenance evidence; it is not a renderer, xAPI conformance certificate, release database row, or storage record.

The publisher boundary is an anti-corruption layer. Target protocols and packaging formats do not become canonical authoring entities. In particular:

- `native_web_publisher` owns `native_cwl_xapi_2_0/v1` validation/finalization and must consume the released shared xAPI 2.0 contract before a full renderer may claim conformance;
- `cmi5_quartz_publisher` owns `cmi5_quartz_xapi_1_0_3/v1` validation and remains a separate adapter;
- cross-contract fallback is rejected;
- SCORM, Common Cartridge, QTI 3.0 reference publication and static HTML remain later target adapters;
- shared interoperability schemas stay in `ContextualWisdomLab/learning-interoperability-contracts` and enter this context only through versioned released contracts.

### Supporting subdomain: Rights & Accessibility Evidence

Rights, accessibility and localization evidence belong to release gating and are referenced by immutable identity. Target adapters consume the approved evidence but do not reinterpret or silently waive it. Publication finalization records stable validation-receipt IDs but does not independently validate or rewrite the external receipt bodies.

### Generic subdomains

Artifact storage, CDN delivery, registry publication, telemetry and deployment are generic capabilities behind explicit ports. They may cache or project immutable release/publication facts but must never become authoring truth.

## Context map

```text
Authoring Surface (Inkspan ACL)
          |
          v
Content Authoring & Release ----> Rights & Accessibility Evidence
          |
          | approved immutable content_release
          v
Publication Admission
          |
          +----> Native Web Renderer (released xAPI 2.0 contract)
          |              |
          |              v
          |       Native Web Finalization
          |              |
          +----> cmi5 Quartz adapter (xAPI 1.0.3 contract)
          +----> later versioned compatibility adapters
                         |
                         v
                Artifact Storage / Delivery
```

Downstream LMS/LRS/assessment systems consume released artifacts/contracts. They do not write Studio authoring tables directly. Shared learning-interoperability contracts are consumed through released schemas rather than copied into this repository.

## Aggregates, entities, value objects and services

Current executable value/domain objects:

- `PublicationRequest` — admission command/value object for one release-target decision;
- `PublicationMetadata` — immutable validated release/contract authority;
- `BlockingFeature` — semantic incompatibility evidence;
- `PublicationOutcome` — trusted admission result;
- `NativeWebPublicationReceipt` — immutable exact-byte publication evidence.

Current domain services:

- `evaluate_publication` — Publication Admission service;
- `finalize_native_web_publication` — Native-Web Publication Finalization service.

Future aggregate roots/entities remain `ContentProject`, `ContentRelease`, release components/assets/approvals and durable publication receipts. No repository abstraction is introduced until persistence exists.

## Persistence direction

No database is introduced by the current kernels. When persistence is implemented, authoritative relational objects use two-or-more-word `snake_case` names and 3NF. Mutable authoring transactions and immutable release/publication receipts use separate transaction boundaries. Immutable release and publication receipt rows are append-only; item-level UPSERT is reserved for mutable authoring/index records with explicit idempotency keys and tests. Read replicas/materialized views may be added only from measured contention or read-load evidence rather than by denormalizing authoritative facts.
