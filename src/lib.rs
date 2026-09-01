//! Deterministic publication admission for approved Learning Content Studio releases.
//!
//! This crate establishes a fail-closed boundary between caller intent and authoritative
//! release/target evidence. Callers may select only a release identity and publisher target;
//! approval, immutable release metadata, target contract identity, and incompatibility evidence
//! must come from explicit authority ports.

/// A target-specific publication path.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PublisherTarget {
    /// Native CWL web activity using the xAPI 2.0 interoperability contract.
    NativeWeb,
    /// cmi5 Quartz publication using the xAPI 1.0.3 interoperability contract.
    Cmi5Quartz,
}

impl PublisherTarget {
    fn required_contract_id(self) -> &'static str {
        match self {
            Self::NativeWeb => "native_cwl_xapi_2_0/v1",
            Self::Cmi5Quartz => "cmi5_quartz_xapi_1_0_3/v1",
        }
    }
}

/// A feature that prevents lossless publication to the selected target.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BlockingFeature {
    /// Stable code identifying the unsupported source feature.
    pub feature_code: String,
    /// Stable reference to the source component containing the feature.
    pub source_component_reference: String,
    /// Stable reason describing why publication must fail closed.
    pub reason_code: String,
}

impl BlockingFeature {
    /// Creates incompatibility evidence.
    #[must_use]
    pub fn new(feature_code: &str, source_component_reference: &str, reason_code: &str) -> Self {
        Self {
            feature_code: feature_code.into(),
            source_component_reference: source_component_reference.into(),
            reason_code: reason_code.into(),
        }
    }
}

/// Caller intent for one publication-admission decision.
///
/// Approval, source identity, locale, publisher contract metadata, and blockers are deliberately
/// absent. Those facts must be supplied by the authority ports passed to [`evaluate_publication`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PublicationRequest {
    content_release_id: String,
    publisher_target: PublisherTarget,
}

impl PublicationRequest {
    /// Creates caller intent for a release and target.
    #[must_use]
    pub fn new(content_release_id: &str, publisher_target: PublisherTarget) -> Self {
        Self {
            content_release_id: content_release_id.into(),
            publisher_target,
        }
    }

    /// Returns the requested immutable content release identity.
    #[must_use]
    pub fn content_release_id(&self) -> &str {
        &self.content_release_id
    }

    /// Returns the requested publisher target.
    #[must_use]
    pub fn publisher_target(&self) -> PublisherTarget {
        self.publisher_target
    }
}

/// Evidence returned by the authoritative immutable-release boundary.
///
/// Implementations of [`ReleaseAuthorityPort`] are security-sensitive anti-corruption adapters:
/// production adapters must derive this evidence from the release authority, never from request
/// fields or synthetic/demo state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReleaseAuthorityEvidence {
    content_release_id: String,
    source_hash: String,
    locale_code: String,
    approved: bool,
    approval_evidence_id: String,
}

impl ReleaseAuthorityEvidence {
    /// Creates evidence emitted by a release-authority adapter.
    #[must_use]
    pub fn new(
        content_release_id: &str,
        source_hash: &str,
        locale_code: &str,
        approved: bool,
        approval_evidence_id: &str,
    ) -> Self {
        Self {
            content_release_id: content_release_id.into(),
            source_hash: source_hash.into(),
            locale_code: locale_code.into(),
            approved,
            approval_evidence_id: approval_evidence_id.into(),
        }
    }

    /// Returns the release identity asserted by the authority.
    #[must_use]
    pub fn content_release_id(&self) -> &str {
        &self.content_release_id
    }

    /// Returns the authority-owned SHA-256 source identity.
    #[must_use]
    pub fn source_hash(&self) -> &str {
        &self.source_hash
    }

    /// Returns the authority-owned locale.
    #[must_use]
    pub fn locale_code(&self) -> &str {
        &self.locale_code
    }

    /// Returns whether the release authority reports completed approval.
    #[must_use]
    pub fn approved(&self) -> bool {
        self.approved
    }

    /// Returns the stable approval evidence identity.
    #[must_use]
    pub fn approval_evidence_id(&self) -> &str {
        &self.approval_evidence_id
    }
}

/// Port owned by the immutable-release authority boundary.
pub trait ReleaseAuthorityPort {
    /// Returns authoritative evidence for the requested release, or `None` when no such
    /// authoritative evidence can be established.
    fn release_evidence(&self, content_release_id: &str) -> Option<ReleaseAuthorityEvidence>;
}

/// Evidence returned by the target-specific compatibility-validation boundary.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TargetCompatibilityEvidence {
    publisher_target: PublisherTarget,
    publisher_contract_id: String,
    publisher_version: String,
    standard_revision: String,
    validation_evidence_id: String,
    blocking_features: Vec<BlockingFeature>,
}

impl TargetCompatibilityEvidence {
    /// Creates evidence emitted by a target-compatibility adapter.
    #[must_use]
    pub fn new(
        publisher_target: PublisherTarget,
        publisher_contract_id: &str,
        publisher_version: &str,
        standard_revision: &str,
        validation_evidence_id: &str,
        blocking_features: Vec<BlockingFeature>,
    ) -> Self {
        Self {
            publisher_target,
            publisher_contract_id: publisher_contract_id.into(),
            publisher_version: publisher_version.into(),
            standard_revision: standard_revision.into(),
            validation_evidence_id: validation_evidence_id.into(),
            blocking_features,
        }
    }

    /// Returns the target validated by this evidence.
    #[must_use]
    pub fn publisher_target(&self) -> PublisherTarget {
        self.publisher_target
    }

    /// Returns the target-owned contract identity.
    #[must_use]
    pub fn publisher_contract_id(&self) -> &str {
        &self.publisher_contract_id
    }

    /// Returns the target-owned publisher version.
    #[must_use]
    pub fn publisher_version(&self) -> &str {
        &self.publisher_version
    }

    /// Returns the target-owned standards revision.
    #[must_use]
    pub fn standard_revision(&self) -> &str {
        &self.standard_revision
    }

    /// Returns the stable compatibility-validation evidence identity.
    #[must_use]
    pub fn validation_evidence_id(&self) -> &str {
        &self.validation_evidence_id
    }

    /// Returns blockers asserted by the target validator.
    #[must_use]
    pub fn blocking_features(&self) -> &[BlockingFeature] {
        &self.blocking_features
    }
}

/// Port owned by the target-specific compatibility boundary.
pub trait TargetCompatibilityPort {
    /// Returns compatibility evidence for the authoritative release and requested target, or
    /// `None` when target validation evidence cannot be established.
    fn compatibility_evidence(
        &self,
        release: &ReleaseAuthorityEvidence,
        target: PublisherTarget,
    ) -> Option<TargetCompatibilityEvidence>;
}

/// Stable metadata preserved by either a compatible or incompatible result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PublicationMetadata {
    content_release_id: String,
    source_hash: String,
    release_approval_evidence_id: String,
    publisher_contract_id: String,
    publisher_version: String,
    standard_revision: String,
    target_validation_evidence_id: String,
    locale_code: String,
}

impl PublicationMetadata {
    /// Returns the immutable content release identity.
    #[must_use]
    pub fn content_release_id(&self) -> &str {
        &self.content_release_id
    }

    /// Returns the authority-owned SHA-256 source identity.
    #[must_use]
    pub fn source_hash(&self) -> &str {
        &self.source_hash
    }

    /// Returns the release-approval evidence identity.
    #[must_use]
    pub fn release_approval_evidence_id(&self) -> &str {
        &self.release_approval_evidence_id
    }

    /// Returns the version-specific publisher contract identity.
    #[must_use]
    pub fn publisher_contract_id(&self) -> &str {
        &self.publisher_contract_id
    }

    /// Returns the publisher implementation/contract version.
    #[must_use]
    pub fn publisher_version(&self) -> &str {
        &self.publisher_version
    }

    /// Returns the explicit target-standard revision.
    #[must_use]
    pub fn standard_revision(&self) -> &str {
        &self.standard_revision
    }

    /// Returns the target-validation evidence identity.
    #[must_use]
    pub fn target_validation_evidence_id(&self) -> &str {
        &self.target_validation_evidence_id
    }

    /// Returns the locale pinned by the immutable-release authority.
    #[must_use]
    pub fn locale_code(&self) -> &str {
        &self.locale_code
    }
}

/// Trust state of a validated publication-admission outcome.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PublicationStatus {
    /// The immutable release may proceed to the selected target-specific publisher.
    Compatible,
    /// Publication must stop because source semantics cannot be preserved.
    Incompatible,
}

/// Deterministic publication-admission outcome created only by [`evaluate_publication`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PublicationOutcome {
    status: PublicationStatus,
    metadata: PublicationMetadata,
    blocking_features: Vec<BlockingFeature>,
}

impl PublicationOutcome {
    /// Returns whether the authority-backed release is compatible or incompatible.
    #[must_use]
    pub fn status(&self) -> PublicationStatus {
        self.status
    }

    /// Returns validated immutable publication authority metadata.
    #[must_use]
    pub fn metadata(&self) -> &PublicationMetadata {
        &self.metadata
    }

    /// Returns canonically ordered authority-provided blocking evidence.
    #[must_use]
    pub fn blocking_features(&self) -> &[BlockingFeature] {
        &self.blocking_features
    }

    /// Serializes the validated admission result into deterministic JSON field and array order.
    #[must_use]
    pub fn canonical_json(&self) -> String {
        let status = match self.status {
            PublicationStatus::Compatible => "compatible",
            PublicationStatus::Incompatible => "incompatible",
        };
        let blockers = self
            .blocking_features
            .iter()
            .map(|feature| {
                format!(
                    "{{\"feature_code\":{},\"source_component_reference\":{},\"reason_code\":{}}}",
                    json_string(&feature.feature_code),
                    json_string(&feature.source_component_reference),
                    json_string(&feature.reason_code),
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        format!(
            "{{\"publication_status\":{},\"content_release_id\":{},\"release_approval_evidence_id\":{},\"publisher_contract_id\":{},\"publisher_version\":{},\"standard_revision\":{},\"target_validation_evidence_id\":{},\"source_hash\":{},\"locale_code\":{},\"blocking_features\":[{}]}}",
            json_string(status),
            json_string(&self.metadata.content_release_id),
            json_string(&self.metadata.release_approval_evidence_id),
            json_string(&self.metadata.publisher_contract_id),
            json_string(&self.metadata.publisher_version),
            json_string(&self.metadata.standard_revision),
            json_string(&self.metadata.target_validation_evidence_id),
            json_string(&self.metadata.source_hash),
            json_string(&self.metadata.locale_code),
            blockers,
        )
    }
}

/// Fail-closed validation errors returned before any publisher may emit an artifact.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AdmissionError {
    /// The release authority could not establish evidence for the requested release.
    ReleaseEvidenceUnavailable,
    /// The release authority returned evidence for another release identity.
    ReleaseAuthorityMismatch,
    /// The release has not completed authoritative approval.
    ReleaseNotApproved,
    /// The target authority could not establish compatibility evidence.
    CompatibilityEvidenceUnavailable,
    /// The target authority returned evidence for another requested target.
    CompatibilityAuthorityMismatch,
    /// The source identity is not a 64-hex SHA-256 digest prefixed by `sha256:`.
    InvalidSourceHash,
    /// The target authority selected a contract not owned by the requested target.
    ContractTargetMismatch,
    /// A required authority or request identity field was empty or whitespace-only.
    EmptyRequiredField(&'static str),
    /// Two blocking-feature entries had the same canonical three-field identity.
    DuplicateBlockingFeature,
}

/// Validates caller publication intent against release and target authority ports.
///
/// The caller cannot assert approval, source identity, locale, target contract/version/standard,
/// or compatibility blockers through [`PublicationRequest`]. Those facts are read from the two
/// supplied authority ports and cross-bound to the requested release/target before an opaque
/// [`PublicationOutcome`] is created.
///
/// # Errors
///
/// Returns [`AdmissionError`] when authority evidence is unavailable, mismatched, unapproved,
/// malformed, cross-target, incomplete, or contains duplicate blocker identities.
pub fn evaluate_publication(
    request: PublicationRequest,
    release_authority: &dyn ReleaseAuthorityPort,
    compatibility_authority: &dyn TargetCompatibilityPort,
) -> Result<PublicationOutcome, AdmissionError> {
    require_non_empty(&request.content_release_id, "content_release_id")?;

    let release = release_authority
        .release_evidence(&request.content_release_id)
        .ok_or(AdmissionError::ReleaseEvidenceUnavailable)?;
    if release.content_release_id != request.content_release_id {
        return Err(AdmissionError::ReleaseAuthorityMismatch);
    }
    if !release.approved {
        return Err(AdmissionError::ReleaseNotApproved);
    }

    require_non_empty(&release.source_hash, "source_hash")?;
    require_non_empty(&release.locale_code, "locale_code")?;
    require_non_empty(
        &release.approval_evidence_id,
        "release_approval_evidence_id",
    )?;
    if !is_sha256_identity(&release.source_hash) {
        return Err(AdmissionError::InvalidSourceHash);
    }

    let mut compatibility = compatibility_authority
        .compatibility_evidence(&release, request.publisher_target)
        .ok_or(AdmissionError::CompatibilityEvidenceUnavailable)?;
    if compatibility.publisher_target != request.publisher_target {
        return Err(AdmissionError::CompatibilityAuthorityMismatch);
    }

    require_non_empty(
        &compatibility.publisher_contract_id,
        "publisher_contract_id",
    )?;
    require_non_empty(&compatibility.publisher_version, "publisher_version")?;
    require_non_empty(&compatibility.standard_revision, "standard_revision")?;
    require_non_empty(
        &compatibility.validation_evidence_id,
        "target_validation_evidence_id",
    )?;
    if compatibility.publisher_contract_id != request.publisher_target.required_contract_id() {
        return Err(AdmissionError::ContractTargetMismatch);
    }

    for feature in &compatibility.blocking_features {
        require_non_empty(&feature.feature_code, "blocking_feature.feature_code")?;
        require_non_empty(
            &feature.source_component_reference,
            "blocking_feature.source_component_reference",
        )?;
        require_non_empty(&feature.reason_code, "blocking_feature.reason_code")?;
    }

    compatibility.blocking_features.sort();
    if compatibility
        .blocking_features
        .windows(2)
        .any(|pair| pair[0] == pair[1])
    {
        return Err(AdmissionError::DuplicateBlockingFeature);
    }

    let metadata = PublicationMetadata {
        content_release_id: release.content_release_id,
        source_hash: release.source_hash,
        release_approval_evidence_id: release.approval_evidence_id,
        publisher_contract_id: compatibility.publisher_contract_id,
        publisher_version: compatibility.publisher_version,
        standard_revision: compatibility.standard_revision,
        target_validation_evidence_id: compatibility.validation_evidence_id,
        locale_code: release.locale_code,
    };
    let status = if compatibility.blocking_features.is_empty() {
        PublicationStatus::Compatible
    } else {
        PublicationStatus::Incompatible
    };

    Ok(PublicationOutcome {
        status,
        metadata,
        blocking_features: compatibility.blocking_features,
    })
}

fn require_non_empty(value: &str, field: &'static str) -> Result<(), AdmissionError> {
    if value.trim().is_empty() {
        Err(AdmissionError::EmptyRequiredField(field))
    } else {
        Ok(())
    }
}

fn is_sha256_identity(value: &str) -> bool {
    let Some(digest) = value.strip_prefix("sha256:") else {
        return false;
    };
    digest.len() == 64 && digest.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn json_string(value: &str) -> String {
    let mut output = String::with_capacity(value.len() + 2);
    output.push('"');
    for character in value.chars() {
        match character {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            '\u{08}' => output.push_str("\\b"),
            '\u{0C}' => output.push_str("\\f"),
            control if control <= '\u{1F}' => {
                const HEX: &[u8; 16] = b"0123456789abcdef";
                let code = control as usize;
                output.push_str("\\u00");
                output.push(char::from(HEX[(code >> 4) & 0x0f]));
                output.push(char::from(HEX[code & 0x0f]));
            }
            other => output.push(other),
        }
    }
    output.push('"');
    output
}
