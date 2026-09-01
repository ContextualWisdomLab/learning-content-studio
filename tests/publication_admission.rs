//! Regression and edge-case contract tests for deterministic publication admission.

use learning_content_studio::{
    AdmissionError, BlockingFeature, PublicationRequest, PublicationStatus, PublisherTarget,
    ReleaseAuthorityEvidence, ReleaseAuthorityPort, TargetCompatibilityEvidence,
    TargetCompatibilityPort, evaluate_publication,
};

#[derive(Clone)]
struct FixedReleaseAuthority {
    evidence: Option<ReleaseAuthorityEvidence>,
}

impl ReleaseAuthorityPort for FixedReleaseAuthority {
    fn release_evidence(&self, _content_release_id: &str) -> Option<ReleaseAuthorityEvidence> {
        self.evidence.clone()
    }
}

#[derive(Clone)]
struct FixedCompatibilityAuthority {
    evidence: Option<TargetCompatibilityEvidence>,
}

impl TargetCompatibilityPort for FixedCompatibilityAuthority {
    fn compatibility_evidence(
        &self,
        _release: &ReleaseAuthorityEvidence,
        _target: PublisherTarget,
    ) -> Option<TargetCompatibilityEvidence> {
        self.evidence.clone()
    }
}

fn request(target: PublisherTarget) -> PublicationRequest {
    PublicationRequest::new("content_release_01", target)
}

fn release(
    content_release_id: &str,
    source_hash: &str,
    locale_code: &str,
    approved: bool,
    approval_evidence_id: &str,
) -> ReleaseAuthorityEvidence {
    ReleaseAuthorityEvidence::new(
        content_release_id,
        source_hash,
        locale_code,
        approved,
        approval_evidence_id,
    )
}

fn approved_release() -> ReleaseAuthorityEvidence {
    release(
        "content_release_01",
        &format!("sha256:{}", "a".repeat(64)),
        "en-US",
        true,
        "release_approval_receipt_01",
    )
}

fn compatibility(
    target: PublisherTarget,
    contract_id: &str,
    version: &str,
    revision: &str,
    evidence_id: &str,
    blockers: Vec<BlockingFeature>,
) -> TargetCompatibilityEvidence {
    TargetCompatibilityEvidence::new(
        target,
        contract_id,
        version,
        revision,
        evidence_id,
        blockers,
    )
}

fn native_compatibility(blockers: Vec<BlockingFeature>) -> TargetCompatibilityEvidence {
    compatibility(
        PublisherTarget::NativeWeb,
        "native_cwl_xapi_2_0/v1",
        "1.0.0",
        "2026-08",
        "target_validation_receipt_01",
        blockers,
    )
}

fn evaluate(
    input: PublicationRequest,
    release_evidence: ReleaseAuthorityEvidence,
    compatibility_evidence: TargetCompatibilityEvidence,
) -> Result<learning_content_studio::PublicationOutcome, AdmissionError> {
    let release_authority = FixedReleaseAuthority {
        evidence: Some(release_evidence),
    };
    let compatibility_authority = FixedCompatibilityAuthority {
        evidence: Some(compatibility_evidence),
    };
    evaluate_publication(input, &release_authority, &compatibility_authority)
}

#[test]
fn request_exposes_intent_only() {
    let input = request(PublisherTarget::NativeWeb);
    assert_eq!(input.content_release_id(), "content_release_01");
    assert_eq!(input.publisher_target(), PublisherTarget::NativeWeb);
}

#[test]
fn authority_evidence_accessors_preserve_exact_values() {
    let release = approved_release();
    assert_eq!(release.content_release_id(), "content_release_01");
    assert_eq!(release.source_hash(), format!("sha256:{}", "a".repeat(64)));
    assert_eq!(release.locale_code(), "en-US");
    assert!(release.approved());
    assert_eq!(release.approval_evidence_id(), "release_approval_receipt_01");

    let target = native_compatibility(Vec::new());
    assert_eq!(target.publisher_target(), PublisherTarget::NativeWeb);
    assert_eq!(target.publisher_contract_id(), "native_cwl_xapi_2_0/v1");
    assert_eq!(target.publisher_version(), "1.0.0");
    assert_eq!(target.standard_revision(), "2026-08");
    assert_eq!(target.validation_evidence_id(), "target_validation_receipt_01");
    assert!(target.blocking_features().is_empty());
}

#[test]
fn rejects_mutable_or_unapproved_authority_evidence() {
    let unapproved = release(
        "content_release_01",
        &format!("sha256:{}", "a".repeat(64)),
        "en-US",
        false,
        "release_approval_receipt_01",
    );
    assert_eq!(
        evaluate(
            request(PublisherTarget::NativeWeb),
            unapproved,
            native_compatibility(Vec::new()),
        ),
        Err(AdmissionError::ReleaseNotApproved)
    );
}

#[test]
fn rejects_cross_target_contract_selection() {
    let wrong_contract = compatibility(
        PublisherTarget::NativeWeb,
        "cmi5_quartz_xapi_1_0_3/v1",
        "1.0.0",
        "2026-08",
        "target_validation_receipt_01",
        Vec::new(),
    );
    assert_eq!(
        evaluate(
            request(PublisherTarget::NativeWeb),
            approved_release(),
            wrong_contract,
        ),
        Err(AdmissionError::ContractTargetMismatch)
    );

    let cmi5 = compatibility(
        PublisherTarget::Cmi5Quartz,
        "cmi5_quartz_xapi_1_0_3/v1",
        "1.0.0",
        "2026-08",
        "target_validation_receipt_02",
        Vec::new(),
    );
    let outcome = evaluate(request(PublisherTarget::Cmi5Quartz), approved_release(), cmi5)
        .expect("cmi5 authority owns its contract");
    assert_eq!(outcome.status(), PublicationStatus::Compatible);
}

#[test]
fn rejects_non_sha256_source_identity() {
    let bad = [
        "a".repeat(64),
        "sha256:not-a-digest".into(),
        format!("sha256:{}g", "a".repeat(63)),
    ];
    for source_hash in bad {
        assert_eq!(
            evaluate(
                request(PublisherTarget::NativeWeb),
                release(
                    "content_release_01",
                    &source_hash,
                    "en-US",
                    true,
                    "release_approval_receipt_01",
                ),
                native_compatibility(Vec::new()),
            ),
            Err(AdmissionError::InvalidSourceHash)
        );
    }
}

#[test]
fn uppercase_sha256_identity_is_accepted_without_rewriting_authority_evidence() {
    let upper = format!("sha256:{}", "A".repeat(64));
    let outcome = evaluate(
        request(PublisherTarget::NativeWeb),
        release(
            "content_release_01",
            &upper,
            "en-US",
            true,
            "release_approval_receipt_01",
        ),
        native_compatibility(Vec::new()),
    )
    .expect("uppercase hexadecimal is valid authority input");
    assert_eq!(outcome.metadata().source_hash(), upper);
}

#[test]
fn incompatibility_is_order_independent_and_machine_readable() {
    let first = BlockingFeature::new(
        "video_caption_missing",
        "component_b",
        "accessibility_evidence_missing",
    );
    let second = BlockingFeature::new(
        "audio_rights_missing",
        "component_a",
        "rights_evidence_missing",
    );
    let left = evaluate(
        request(PublisherTarget::NativeWeb),
        approved_release(),
        native_compatibility(vec![first.clone(), second.clone()]),
    )
    .expect("valid incompatible result");
    let right = evaluate(
        request(PublisherTarget::NativeWeb),
        approved_release(),
        native_compatibility(vec![second, first]),
    )
    .expect("valid incompatible result");

    assert_eq!(left, right);
    assert_eq!(left.status(), PublicationStatus::Incompatible);
    assert_eq!(left.blocking_features().len(), 2);
    assert_eq!(left.canonical_json(), right.canonical_json());
    assert!(
        left.canonical_json()
            .contains("\"publication_status\":\"incompatible\"")
    );
}

#[test]
fn authority_empty_blocker_evidence_becomes_compatible() {
    let outcome = evaluate(
        request(PublisherTarget::NativeWeb),
        approved_release(),
        native_compatibility(Vec::new()),
    )
    .expect("valid compatible admission");
    assert_eq!(outcome.status(), PublicationStatus::Compatible);
    assert!(outcome.blocking_features().is_empty());
}

#[test]
fn rejects_duplicate_blocking_feature_identity() {
    let duplicate = BlockingFeature::new(
        "unsupported_feature",
        "component_7",
        "semantic_loss_required",
    );
    assert_eq!(
        evaluate(
            request(PublisherTarget::NativeWeb),
            approved_release(),
            native_compatibility(vec![duplicate.clone(), duplicate]),
        ),
        Err(AdmissionError::DuplicateBlockingFeature)
    );
}

#[test]
fn compatible_admission_preserves_exact_authority_traceability() {
    let outcome = evaluate(
        request(PublisherTarget::NativeWeb),
        approved_release(),
        native_compatibility(Vec::new()),
    )
    .expect("compatible admission");
    let metadata = outcome.metadata();
    assert_eq!(metadata.content_release_id(), "content_release_01");
    assert_eq!(metadata.source_hash(), format!("sha256:{}", "a".repeat(64)));
    assert_eq!(
        metadata.release_approval_evidence_id(),
        "release_approval_receipt_01"
    );
    assert_eq!(metadata.publisher_contract_id(), "native_cwl_xapi_2_0/v1");
    assert_eq!(metadata.publisher_version(), "1.0.0");
    assert_eq!(metadata.standard_revision(), "2026-08");
    assert_eq!(
        metadata.target_validation_evidence_id(),
        "target_validation_receipt_01"
    );
    assert_eq!(metadata.locale_code(), "en-US");

    assert_eq!(
        outcome.canonical_json(),
        format!(
            "{{\"publication_status\":\"compatible\",\"content_release_id\":\"content_release_01\",\"release_approval_evidence_id\":\"release_approval_receipt_01\",\"publisher_contract_id\":\"native_cwl_xapi_2_0/v1\",\"publisher_version\":\"1.0.0\",\"standard_revision\":\"2026-08\",\"target_validation_evidence_id\":\"target_validation_receipt_01\",\"source_hash\":\"sha256:{}\",\"locale_code\":\"en-US\",\"blocking_features\":[]}}",
            "a".repeat(64)
        )
    );
}

#[test]
fn canonical_json_escapes_all_json_control_classes() {
    let outcome = evaluate(
        PublicationRequest::new("q\"\\\n\r\t\u{08}\u{0c}\u{01}é", PublisherTarget::NativeWeb),
        release(
            "q\"\\\n\r\t\u{08}\u{0c}\u{01}é",
            &format!("sha256:{}", "a".repeat(64)),
            "en-US",
            true,
            "release_approval_receipt_01",
        ),
        native_compatibility(Vec::new()),
    )
    .expect("valid escaped identity");
    assert!(
        outcome
            .canonical_json()
            .contains("\"content_release_id\":\"q\\\"\\\\\\n\\r\\t\\b\\f\\u0001é\"")
    );
}

#[test]
fn rejects_empty_request_or_release_authority_fields() {
    let empty_request = PublicationRequest::new(" ", PublisherTarget::NativeWeb);
    assert_eq!(
        evaluate(empty_request, approved_release(), native_compatibility(Vec::new())),
        Err(AdmissionError::EmptyRequiredField("content_release_id"))
    );

    let cases = [
        (" \t", "en-US", "release_approval_receipt_01", "source_hash"),
        (
            &format!("sha256:{}", "a".repeat(64)),
            " ",
            "release_approval_receipt_01",
            "locale_code",
        ),
        (
            &format!("sha256:{}", "a".repeat(64)),
            "en-US",
            " ",
            "release_approval_evidence_id",
        ),
    ];
    for (source_hash, locale, approval_id, field) in cases {
        assert_eq!(
            evaluate(
                request(PublisherTarget::NativeWeb),
                release("content_release_01", source_hash, locale, true, approval_id),
                native_compatibility(Vec::new()),
            ),
            Err(AdmissionError::EmptyRequiredField(field))
        );
    }
}

#[test]
fn rejects_empty_target_authority_fields() {
    let cases = [
        (" ", "1.0.0", "2026-08", "target_validation_receipt_01", "publisher_contract_id"),
        ("native_cwl_xapi_2_0/v1", " ", "2026-08", "target_validation_receipt_01", "publisher_version"),
        ("native_cwl_xapi_2_0/v1", "1.0.0", " ", "target_validation_receipt_01", "standard_revision"),
        ("native_cwl_xapi_2_0/v1", "1.0.0", "2026-08", " ", "target_validation_evidence_id"),
    ];
    for (contract, version, revision, evidence_id, field) in cases {
        assert_eq!(
            evaluate(
                request(PublisherTarget::NativeWeb),
                approved_release(),
                compatibility(
                    PublisherTarget::NativeWeb,
                    contract,
                    version,
                    revision,
                    evidence_id,
                    Vec::new(),
                ),
            ),
            Err(AdmissionError::EmptyRequiredField(field))
        );
    }
}

#[test]
fn rejects_every_empty_blocking_feature_identity_field() {
    let cases = [
        (
            BlockingFeature::new(" ", "component_1", "reason_1"),
            "blocking_feature.feature_code",
        ),
        (
            BlockingFeature::new("feature_1", " ", "reason_1"),
            "blocking_feature.source_component_reference",
        ),
        (
            BlockingFeature::new("feature_1", "component_1", " "),
            "blocking_feature.reason_code",
        ),
    ];
    for (blocker, field) in cases {
        assert_eq!(
            evaluate(
                request(PublisherTarget::NativeWeb),
                approved_release(),
                native_compatibility(vec![blocker]),
            ),
            Err(AdmissionError::EmptyRequiredField(field))
        );
    }
}
