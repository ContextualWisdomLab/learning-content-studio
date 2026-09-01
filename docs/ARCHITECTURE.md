# Architecture

The Learning Content Studio is the LCMS and authoring authority. It owns mutable authoring projects, reusable learning objects, revisions, accessibility variants, localization, rights metadata, review/approval state, and immutable content releases.

It does not own enrollment, learner completion, xAPI statements, psychometric response data, or commercial payment truth.

## Authoring pipeline

Mutable authoring source -> review -> accessibility validation -> rights validation -> approval -> immutable content release -> Publication Admission -> target-specific publication artifact.

## Domain-driven design

### Core subdomain: Content Authoring & Release

The core bounded context owns the lifecycle from mutable authoring work through immutable approval. Its future aggregate roots are `ContentProject` for mutable editorial transactions and `ContentRelease` for immutable publication authority. A release never mutates after approval; corrections create a successor release and preserve lineage.

### Supporting subdomain: Publication Admission & Projection

`src/lib.rs` implements the first executable boundary. `PublicationRequest` is a minimal transaction boundary for one approved immutable release and one publisher target. `evaluate_publication` enforces release approval, SHA-256 source identity, target-specific contract ownership, required identity fields, canonical blocker ordering, and duplicate-blocker rejection.

`PublicationOutcome` is either compatible or incompatible. Compatible admission is permission to proceed to a target adapter, not evidence that an artifact was built or certified. Incompatible admission is deterministic machine-readable evidence that publication would lose required semantics.

The publisher boundary is an anti-corruption layer. Target protocols and packaging formats do not become canonical authoring entities. In particular:

- `native_web_publisher` owns `native_cwl_xapi_2_0/v1` validation;
- `cmi5_quartz_publisher` owns `cmi5_quartz_xapi_1_0_3/v1` validation;
- cross-contract fallback is rejected;
- SCORM, Common Cartridge, QTI 3.0 reference publication and static HTML remain later target adapters.

### Supporting subdomain: Rights & Accessibility Evidence

Rights, accessibility and localization evidence belong to release gating and are referenced by immutable identity. Target adapters consume the approved evidence but do not reinterpret or silently waive it.

### Generic subdomains

Artifact storage, CDN delivery, registry publication, telemetry and deployment are generic capabilities behind explicit ports. They may cache or project immutable release facts but must never become authoring truth.

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
          +----> Native Web adapter (xAPI 2.0 contract)
          +----> cmi5 Quartz adapter (xAPI 1.0.3 contract)
          +----> later versioned compatibility adapters
          |
          v
Artifact Storage / Delivery
```

Downstream LMS/LRS/assessment systems consume released artifacts/contracts. They do not write Studio authoring tables directly. Shared learning-interoperability contracts are consumed through released schemas rather than copied into this repository.

## Initial modules

- structured content model
- reusable component registry
- asset library
- revision control
- accessibility validator
- localization workflow
- rights management
- release pipeline
- Publication Admission kernel
- publisher adapters

Inkspan is the preferred reusable authoring surface; learning-specific semantics live in this repository rather than being copied into Inkspan.

## Persistence direction

No database is introduced by the first admission slice. When persistence is implemented, authoritative relational objects use two-or-more-word `snake_case` names and 3NF. Mutable authoring transactions and immutable release/publication receipts use separate transaction boundaries. Immutable release rows are append-only; item-level UPSERT is reserved for mutable authoring/index records with explicit idempotency keys. Read replicas/materialized views may be added only from measured contention or read-load evidence rather than by denormalizing authoritative facts.
