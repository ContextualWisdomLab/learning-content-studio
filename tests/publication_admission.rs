//! Regression and edge-case contract tests for deterministic publication admission.

use learning_content_studio::{
    AdmissionError, BlockingFeature, PublicationOutcome, PublicationRequest, PublisherTarget,
    evaluate_publication,
};

fn request(target: PublisherTarget, contract_id: &str) -> PublicationRequest {
    PublicationRequest {
        content_release_id: "content_release_01".into(),
        source_hash: format!("sha256:{}", "a".repeat(64)),
        publisher_target: target,
        publisher_contract_id: contract_id.into(),
        publisher_version: "1.0.0".into(),
        standard_revision: "2026-08".into(),
        locale_code: "en-US".into(),
        approved: true,
        blocking_features: Vec::new(),
    }
}

#[test]
fn rejects_mutable_or_unapproved_release() {
    let mut input = request(PublisherTarget::NativeWeb, "native_cwl_xapi_2_0/v1");
    input.approved = false;

    assert_eq!(
        evaluate_publication(input),
        Err(AdmissionError::ReleaseNotApproved)
    );
}

#[test]
fn rejects_cross_target_contract_selection() {
    let input = request(PublisherTarget::Cmi5Quartz, "native_cwl_xapi_2_0/v1");

    assert_eq!(
        evaluate_publication(input),
        Err(AdmissionError::ContractTargetMismatch)
    );
}

#[test]
fn rejects_non_sha256_source_identity() {
    let mut input = request(PublisherTarget::NativeWeb, "native_cwl_xapi_2_0/v1");
    input.source_hash = "sha256:not-a-digest".into();

    assert_eq!(
        evaluate_publication(input),
        Err(AdmissionError::InvalidSourceHash)
    );
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

    let mut left = request(PublisherTarget::Cmi5Quartz, "cmi5_quartz_xapi_1_0_3/v1");
    left.blocking_features = vec![first.clone(), second.clone()];

    let mut right = request(PublisherTarget::Cmi5Quartz, "cmi5_quartz_xapi_1_0_3/v1");
    right.blocking_features = vec![second, first];

    let left_outcome = evaluate_publication(left).expect("valid incompatible result");
    let right_outcome = evaluate_publication(right).expect("valid incompatible result");

    assert_eq!(left_outcome, right_outcome);
    assert_eq!(
        left_outcome.canonical_json(),
        right_outcome.canonical_json()
    );
    assert!(
        left_outcome
            .canonical_json()
            .contains("\"publication_status\":\"incompatible\"")
    );
}

#[test]
fn rejects_duplicate_blocking_feature_identity() {
    let duplicate = BlockingFeature::new(
        "unsupported_feature",
        "component_7",
        "semantic_loss_required",
    );
    let mut input = request(PublisherTarget::NativeWeb, "native_cwl_xapi_2_0/v1");
    input.blocking_features = vec![duplicate.clone(), duplicate];

    assert_eq!(
        evaluate_publication(input),
        Err(AdmissionError::DuplicateBlockingFeature)
    );
}

#[test]
fn compatible_admission_preserves_exact_release_authority() {
    let input = request(PublisherTarget::NativeWeb, "native_cwl_xapi_2_0/v1");

    let outcome = evaluate_publication(input).expect("compatible admission");
    match &outcome {
        PublicationOutcome::Compatible(metadata) => {
            assert_eq!(metadata.content_release_id, "content_release_01");
            assert_eq!(metadata.publisher_contract_id, "native_cwl_xapi_2_0/v1");
            assert_eq!(metadata.locale_code, "en-US");
        }
        PublicationOutcome::Incompatible { .. } => panic!("expected compatible outcome"),
    }

    assert_eq!(
        outcome.canonical_json(),
        format!(
            "{{\"publication_status\":\"compatible\",\"content_release_id\":\"content_release_01\",\"publisher_contract_id\":\"native_cwl_xapi_2_0/v1\",\"publisher_version\":\"1.0.0\",\"standard_revision\":\"2026-08\",\"source_hash\":\"sha256:{}\",\"locale_code\":\"en-US\",\"blocking_features\":[]}}",
            "a".repeat(64)
        )
    );
}

#[test]
fn rejects_empty_required_identity_fields() {
    let mut input = request(PublisherTarget::NativeWeb, "native_cwl_xapi_2_0/v1");
    input.locale_code = " ".into();

    assert_eq!(
        evaluate_publication(input),
        Err(AdmissionError::EmptyRequiredField("locale_code"))
    );
}
