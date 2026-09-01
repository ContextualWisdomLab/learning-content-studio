# Standards traceability

Initial content and accessibility profile. Adoption does not imply conformance; executable evidence is required before any conformance or certification claim.

- **WCAG 2.2 Level AA / ISO/IEC 40500:2025**: both the Studio authoring UI and generated learner-facing web content target WCAG 2.2 Level AA.
- **ATAG 2.0**: authoring workflows follow ATAG 2.0 principles: the authoring UI itself must be accessible, and the tool must support authors in producing accessible output.
- **ISO/IEC 24751-1:2008, Part 1 — Framework and reference model**: framework for matching learner needs/preferences with resource descriptions.
- **ISO/IEC 24751-2:2008, Part 2 — Access for all personal needs and preferences for digital delivery**: adopted for explicit learner needs/preferences references; Studio source metadata stores only authorized functional preference references, not inferred disability labels.
- **ISO/IEC 24751-3:2008, Part 3 — Access for all digital resource description**: adopted for source/release metadata describing accessibility characteristics and alternative resources; publisher output must preserve supported resource-description semantics or report incompatibility.
- **ISO/IEC 24751-4:2023, Part 4 — Access for all framework registry server API**: planned only for registry interoperability; no conformance claim until a registry adapter exists.
- **ISO/IEC 19788-1:2024, Part 1 — Framework**: used as the framework for defining learning-resource metadata application profiles; it is not itself the Studio's canonical application profile. The CWL-owned profile identifier is `cwl_learning_resource_metadata/v1`, whose required fields and mappings must be specified in a later contract PR before use.
- **QTI 3.0**: assessment-content interchange. This baseline supports **QTI 3.0 reference-only artifacts** that bind approved assessment references/metadata; it does not claim full QTI package publication.
- **cmi5 Quartz, 1st Edition**: version-pinned package publisher for LMS-managed launch, with an explicit xAPI 1.0.3 compatibility contract.
- **SCORM 1.2 and SCORM 2004**: legacy compatibility publishers only, never the canonical authoring model.
- **Common Cartridge**: import/export adapter rather than internal source format.

Every publisher must preserve provenance and report semantic loss as an explicit incompatibility rather than degrading content silently. Evidence status remains `Not evidenced` for every target until exact-revision conformance tests and CI receipts are linked here.
