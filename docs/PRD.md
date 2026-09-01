# Product requirements — Learning Content Studio

## Product outcome

Learning Content Studio gives learning teams one authoritative place to author, review, approve and publish reusable learning content without treating delivery packages as editable source. A customer must be able to know exactly which approved release produced an artifact, which rights/accessibility evidence allowed publication, and why a target was rejected when semantics cannot be preserved.

## Customer workflow

1. Author or revise structured learning content and assets.
2. Complete semantic, accessibility, localization and rights review.
3. Approve an immutable `content_release`.
4. Select a versioned publisher target.
5. Run Publication Admission.
6. If compatible, create a deterministic target artifact and publication receipt; if incompatible, return stable machine-readable blocking evidence.
7. Downstream learning systems consume the released projection without acquiring authoring authority.

## Current commercial slice

The executable `Publication Admission` kernel is the first production surface. It supports two distinct contract boundaries:

- native CWL web activity → `native_cwl_xapi_2_0/v1`;
- cmi5 Quartz → `cmi5_quartz_xapi_1_0_3/v1`.

It does not yet emit packages and therefore makes no runtime-conformance or certification claim.

## Product invariants

- mutable authoring state can never masquerade as an approved release;
- target adapters cannot rewrite canonical authoring truth;
- unsupported semantics fail closed rather than disappear;
- native xAPI 2.0 and cmi5/xAPI 1.0.3 contracts never silently cross-select;
- identical admission input yields an identical compatible/incompatible result;
- publisher artifacts must later be reproducible from exact release bytes and exact contract/version;
- real customer PII, institutional names or protected third-party content are absent from public fixtures;
- downstream LMS, LRS, psychometrics and billing systems retain their own truth boundaries.

## Release gates

A buyer-facing release requires protected-branch integration, exact-head tests/security/review evidence, reproducible artifacts, provenance/SBOM where applicable, rights/accessibility evidence, documented operability, and a completed end-to-end author→review→approve→publish workflow. Browser/UI claims additionally require WCAG 2.2 AA/ATAG 2.0 evidence and realistic interaction verification.

## Deferred capabilities

Immutable persistence, actual native-web/cmi5 artifact generation, SCORM/Common Cartridge/QTI 3.0 reference adapters, authoring UI, content storage, deployment, load testing, and CEFR task/rubric specialization remain explicit gaps tracked in `docs/product-technical-gap-baseline.md`.
