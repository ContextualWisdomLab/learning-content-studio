//! Deterministic publication admission for approved Learning Content Studio releases.
//!
//! This crate is deliberately narrower than a complete publisher. It establishes the
//! fail-closed boundary that every target-specific publisher must cross before emitting
//! artifacts: the release must be approved, its source identity must be a SHA-256 digest,
//! the target must select its own version-specific interoperability contract, and any
//! incompatibility evidence must be canonically ordered.

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

/// Exact immutable inputs to the publication-admission decision.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PublicationRequest {
    /// Immutable content release identity.
    pub content_release_id: String,
    /// Caller-supplied SHA-256 identity of the canonical immutable release bytes.
    ///
    /// Admission validates digest syntax only. A byte-producing publisher must recompute and
    /// verify this identity against the exact immutable release bytes before emission.
    pub source_hash: String,
    /// Publisher target selected by the caller.
    pub publisher_target: PublisherTarget,
    /// Version-specific publisher contract identity.
    pub publisher_contract_id: String,
    /// Publisher implementation/contract version.
    pub publisher_version: String,
    /// Explicit target-standard revision.
    pub standard_revision: String,
    /// Locale pinned in the immutable release.
    pub locale_code: String,
    /// Whether the immutable release has completed approval.
    pub approved: bool,
    /// Explicit semantic blockers found before target transformation.
    pub blocking_features: Vec<BlockingFeature>,
}

/// Stable metadata preserved by either a compatible or incompatible result.
///
/// Fields are intentionally private so callers cannot manufacture metadata that appears to
/// have passed publication admission. Read-only accessors expose the validated authority.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PublicationMetadata {
    content_release_id: String,
    source_hash: String,
    publisher_contract_id: String,
    publisher_version: String,
    standard_revision: String,
    locale_code: String,
}

impl PublicationMetadata {
    /// Returns the immutable content release identity.
    #[must_use]
    pub fn content_release_id(&self) -> &str {
        &self.content_release_id
    }

    /// Returns the syntax-validated caller-supplied SHA-256 identity.
    #[must_use]
    pub fn source_hash(&self) -> &str {
        &self.source_hash
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

    /// Returns the locale pinned in the immutable release.
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
///
/// The fields are private by design. External callers can inspect validated state through
/// read-only accessors but cannot construct or mutate a trusted outcome directly.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PublicationOutcome {
    status: PublicationStatus,
    metadata: PublicationMetadata,
    blocking_features: Vec<BlockingFeature>,
}

impl PublicationOutcome {
    /// Returns whether the validated release is compatible or incompatible with the target.
    #[must_use]
    pub fn status(&self) -> PublicationStatus {
        self.status
    }

    /// Returns validated immutable publication authority metadata.
    #[must_use]
    pub fn metadata(&self) -> &PublicationMetadata {
        &self.metadata
    }

    /// Returns canonically ordered blocking evidence.
    ///
    /// Compatible outcomes always return an empty slice; incompatible outcomes always return
    /// at least one blocking feature.
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
            "{{\"publication_status\":{},\"content_release_id\":{},\"publisher_contract_id\":{},\"publisher_version\":{},\"standard_revision\":{},\"source_hash\":{},\"locale_code\":{},\"blocking_features\":[{}]}}",
            json_string(status),
            json_string(&self.metadata.content_release_id),
            json_string(&self.metadata.publisher_contract_id),
            json_string(&self.metadata.publisher_version),
            json_string(&self.metadata.standard_revision),
            json_string(&self.metadata.source_hash),
            json_string(&self.metadata.locale_code),
            blockers,
        )
    }
}

/// Fail-closed validation errors returned before any publisher may emit an artifact.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AdmissionError {
    /// The release has not completed approval and is therefore mutable/not publishable.
    ReleaseNotApproved,
    /// The source identity is not a lowercase or uppercase 64-hex SHA-256 digest.
    InvalidSourceHash,
    /// The chosen target attempted to use another target's interoperability contract.
    ContractTargetMismatch,
    /// A required identity field was empty or whitespace-only.
    EmptyRequiredField(&'static str),
    /// Two blocking-feature entries had the same canonical three-field identity.
    DuplicateBlockingFeature,
}

/// Validates an immutable release at the target-specific publication boundary.
///
/// The function never converts one publisher contract into another. Incompatibility
/// entries are sorted by `feature_code`, `source_component_reference`, then
/// `reason_code`, and exact duplicate triples are rejected. The source hash is checked
/// for SHA-256 syntax here; byte-to-digest verification belongs to the downstream
/// byte-producing publisher boundary. Successful return values cannot be constructed or
/// mutated by downstream callers and therefore remain proof that this validation ran.
///
/// # Errors
///
/// Returns [`AdmissionError`] when release approval, source identity, required fields,
/// target/contract ownership, or blocking-feature uniqueness is invalid.
pub fn evaluate_publication(
    mut request: PublicationRequest,
) -> Result<PublicationOutcome, AdmissionError> {
    if !request.approved {
        return Err(AdmissionError::ReleaseNotApproved);
    }

    require_non_empty(&request.content_release_id, "content_release_id")?;
    require_non_empty(&request.source_hash, "source_hash")?;
    require_non_empty(&request.publisher_contract_id, "publisher_contract_id")?;
    require_non_empty(&request.publisher_version, "publisher_version")?;
    require_non_empty(&request.standard_revision, "standard_revision")?;
    require_non_empty(&request.locale_code, "locale_code")?;

    if !is_sha256_identity(&request.source_hash) {
        return Err(AdmissionError::InvalidSourceHash);
    }

    if request.publisher_contract_id != request.publisher_target.required_contract_id() {
        return Err(AdmissionError::ContractTargetMismatch);
    }

    for feature in &request.blocking_features {
        require_non_empty(&feature.feature_code, "blocking_feature.feature_code")?;
        require_non_empty(
            &feature.source_component_reference,
            "blocking_feature.source_component_reference",
        )?;
        require_non_empty(&feature.reason_code, "blocking_feature.reason_code")?;
    }

    request.blocking_features.sort();
    if request
        .blocking_features
        .windows(2)
        .any(|pair| pair[0] == pair[1])
    {
        return Err(AdmissionError::DuplicateBlockingFeature);
    }

    let metadata = PublicationMetadata {
        content_release_id: request.content_release_id,
        source_hash: request.source_hash,
        publisher_contract_id: request.publisher_contract_id,
        publisher_version: request.publisher_version,
        standard_revision: request.standard_revision,
        locale_code: request.locale_code,
    };
    let status = if request.blocking_features.is_empty() {
        PublicationStatus::Compatible
    } else {
        PublicationStatus::Incompatible
    };

    Ok(PublicationOutcome {
        status,
        metadata,
        blocking_features: request.blocking_features,
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
