# Publishing contract

Authoring sources are not delivery packages. Publication is an explicit, deterministic transformation from an approved immutable content release.

Initial publisher targets:

- `native_web_publisher`
- `cmi5_quartz_publisher`
- `scorm_1_2_publisher`
- `scorm_2004_publisher`
- `common_cartridge_publisher`
- `qti_reference_publisher`
- `static_html_publisher`

A publisher must never silently discard semantics. Unsupported source features produce a machine-readable incompatible result identifying blocking features. Published artifacts carry content release ID, publisher version, standard revision, source hash, artifact hash, and validation evidence.

cmi5 Quartz output is explicitly tied to xAPI 1.0.3 compatibility. Native CWL learning activity targets xAPI 2.0 through the shared interoperability contracts.
