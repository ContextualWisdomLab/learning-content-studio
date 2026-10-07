# ADR 0003: Durable publication receipt evidence store

- Status: Proposed
- Date: 2026-10-01

## Problem

`NativeWebPublicationReceipt` was deterministic but existed only in process memory. A process restart could therefore erase the exact evidence that binds one approved release and publisher contract to emitted artifact and build-manifest hashes. The repository still lacks an authoritative relational release/audit store, and inventing that aggregate in a child publication slice would violate the Content Authoring & Release single-writer boundary.

## Constraints

- preserve exact canonical receipt bytes without acquiring release approval or target-validation authority;
- never overwrite different bytes for the same release/contract identity;
- survive process restart and interrupted temporary writes;
- use the Rust standard library and existing pinned SHA-256 dependency only;
- keep the adapter bounded so future 3NF authority can replace it behind a repository boundary;
- retain exact-head 100% production statement/region, line, and branch coverage.

## Decision

Add `FilePublicationReceiptStore` as an append-only evidence repository after native byte finalization. The key is the exact `content_release_id` plus NUL plus `publisher_contract_id`; the filename is its SHA-256. A versioned record contains the identity digest, payload digest, and exact canonical receipt bytes.

Persistence writes and fsyncs a hidden temporary record in the target directory, atomically hard-links it to the final no-overwrite name, removes the temporary name, and fsyncs the store directory. A newly created store leaf fsyncs its already-provisioned parent. Identical replay also fsyncs the directory before returning success. Different bytes for an existing identity fail closed. A colliding PID/sequence temporary candidate is preserved and skipped because another PID namespace may own it; orphan temporaries remain invisible.

The deployment trust boundary is a same-filesystem, privately writable directory. Digests detect corruption and identity substitution but do not defend against a privileged writer that can replace a record and recompute its digests.

## Alternatives

- **PostgreSQL authority now:** rejected for this slice because transaction/schema/retention/audit ownership is not yet specified and would conflate receipt evidence with the future `content_release` aggregate.
- **Overwrite with rename:** rejected because standard rename can replace existing evidence and hide same-identity conflicts.
- **Memory-only cache:** rejected because it does not survive restart and provides no durable buyer-visible evidence.
- **Copy an ecosystem Core or read another service database:** rejected because no immutable released owner contract exists and cross-service source/SQL access would violate the boundary.

## Evidence and verification

Tests first established write/reopen/read exactness, identical replay, conflicting replay, and orphan recovery. Review then found replay-directory and newly-created-parent fsync gaps plus PID-reuse collision and a scheduling-dependent test; deterministic failure injection and bounded filesystem tests repair those paths. The exact workflow requires every production source file to reach 100% statement/region, line, and branch coverage plus fmt, locked Clippy/tests, and rustdoc.

## Consequences and risks

The product can retain exact receipt evidence across restart without claiming authoritative release persistence. Operators must provision access controls, backup, retention, capacity limits, and recovery monitoring. Unbounded or hostile record sizes, symlink/privileged-writer attacks, remote filesystems without expected hard-link/fsync semantics, artifact storage, and relational audit transactions remain explicitly open.

## Operational and failure scenes

- A publisher retries the same receipt after an uncertain response: the store verifies exact bytes, fsyncs the directory, and returns `AlreadyPresent`.
- A retry presents different bytes for the same release/contract: the original remains untouched and the request fails.
- A process dies after creating a hidden temporary file: the orphan is not visible as committed evidence and a later process skips the colliding name without deleting a potentially active writer's file.
- A committed record is truncated, copied under another identity, or payload-tampered: load fails closed as corruption.
- Storage or directory sync fails: no success is reported; already-linked evidence remains available for an idempotent repair retry.

## Follow-up

Define the authoritative `content_release` / `publication_receipt` transaction model, resource limits, retention/recovery SLOs, and service ACL before promoting persistence beyond this bounded adapter. Keep this ADR Proposed until PR integration and protected exact-head evidence are complete.
