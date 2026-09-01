# Standards traceability

Initial content, accessibility and publication-evidence profile. Adoption does not imply conformance; executable evidence is required before any conformance or certification claim.

- **WCAG 2.2 Level AA / ISO/IEC 40500:2025**: both the Studio authoring UI and generated learner-facing web content target WCAG 2.2 Level AA.
- **ATAG 2.0**: authoring workflows follow ATAG 2.0 principles: the authoring UI itself must be accessible, and the tool must support authors in producing accessible output.
- **ISO/IEC 24751-1:2008, Part 1 — Framework and reference model**: framework for matching learner needs/preferences with resource descriptions.
- **ISO/IEC 24751-2:2008, Part 2 — Access for all personal needs and preferences for digital delivery**: adopted for explicit learner needs/preferences references; Studio source metadata stores only authorized functional preference references, not inferred disability labels.
- **ISO/IEC 24751-3:2008, Part 3 — Access for all digital resource description**: adopted for source/release metadata describing accessibility characteristics and alternative resources; publisher output must preserve supported resource-description semantics or report incompatibility.
- **ISO/IEC 24751-4:2023, Part 4 — Access for all framework registry server API**: planned only for registry interoperability; no conformance claim until a registry adapter exists.
- **ISO/IEC 19788-1:2024, Part 1 — Framework**: used as the framework for defining learning-resource metadata application profiles; it is not itself the Studio's canonical application profile. The CWL-owned profile identifier is `cwl_learning_resource_metadata/v1`, whose required fields and mappings must be specified in a later contract PR before use.
- **NIST FIPS 180-4 — Secure Hash Standard**: SHA-256 is the publication byte-identity algorithm for `source_hash`, `artifact_hash` and `build_manifest_hash`. Admission checks SHA-256 syntax only; `finalize_native_web_publication` recomputes SHA-256 from the exact canonical release bytes and exact emitted byte sets before creating a trusted receipt. NIST announced that FIPS 180-4 will be revised, but its current CAVP material still lists SHA-256 under FIPS 180-4; this repository must re-check the normative revision before a release that changes the digest contract.
- **RustCrypto `sha2` 0.11.0**: implementation dependency for SHA-256. It was released 2026-03-25 and is pinned exactly in `Cargo.toml`; this is implementation evidence, not a NIST algorithm-validation or certification claim. Protected dependency/security evidence remains mandatory before integration.
- **QTI 3.0**: assessment-content interchange. This baseline supports **QTI 3.0 reference-only artifacts** that bind approved assessment references/metadata; it does not claim full QTI package publication.
- **cmi5 Quartz, 1st Edition**: version-pinned package publisher for LMS-managed launch, with an explicit xAPI 1.0.3 compatibility contract.
- **SCORM 1.2 and SCORM 2004**: legacy compatibility publishers only, never the canonical authoring model.
- **Common Cartridge**: import/export adapter rather than internal source format.

Every publisher must preserve provenance and report semantic loss as an explicit incompatibility rather than degrading content silently. Evidence status remains `Not evidenced` for interoperability targets until exact-revision conformance tests and CI receipts are linked here. The native-web byte-finalization kernel provides provenance/hash evidence only; it is not xAPI 2.0 conformance evidence and it does not establish FIPS module certification.

## Publication-byte evidence mapping

| Requirement/evidence source | Implementation | Executable evidence | Claim boundary |
| --- | --- | --- | --- |
| FIPS 180-4 SHA-256 message digest | `sha256_identity` via RustCrypto `Sha256::digest` | `tests/native_web_publication.rs` known digest fixtures | Correct algorithm contract under tests; no FIPS module certification claim |
| admitted source identity equals exact canonical release bytes | `finalize_native_web_publication` source recomputation | source mismatch and uppercase-hex equivalence tests | Byte equality only; admission itself remains syntax-only |
| exact emitted artifact identity | `artifact_hash` computed inside finalizer | known artifact digest fixture and repeatability test | No rendering/conformance claim |
| exact build-manifest identity | `build_manifest_hash` computed inside finalizer | known manifest digest fixture and repeatability test | No package-format conformance claim |
| validation evidence order is deterministic | lexical sort + duplicate rejection | caller-order equivalence, empty/duplicate ID tests | Evidence identity only; does not validate the referenced receipt content |

## References (APA 7th)

National Institute of Standards and Technology. (2015). *Secure Hash Standard (SHS) (FIPS PUB 180-4).* U.S. Department of Commerce. https://doi.org/10.6028/NIST.FIPS.180-4

National Institute of Standards and Technology. (2023, March 7; updated 2025, February 3). *Decision to revise FIPS 180-4, Secure Hash Standard (SHS).* https://www.nist.gov/news-events/news/2023/03/decision-revise-fips-180-4-secure-hash-standard-shs

RustCrypto Developers. (2026, March 25). *sha2 0.11.0* [Rust crate]. Docs.rs. https://docs.rs/crate/sha2/0.11.0
