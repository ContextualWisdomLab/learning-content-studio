//! Regression and edge-case contract tests for deterministic publication admission.

use learning_content_studio::{
    AdmissionError, BlockingFeature, PublicationMetadata, PublicationOutcome, PublicationRequest,
    PublisherTarget, evaluate_publication,
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

fn metadata() -> PublicationMetadata {
    PublicationMetadata {
        content_release_id: "content_release_01".into(),
        source_hash: format!("sha256:{}", "a".repeat(64)),
        publisher_contract_id: "native_cwl_xapi_2_0/v1".into(),
        publisher_version: "1.0.0".into(),
        standard_revision: "2026-08".into(),
        locale_code: "en-US".into(),
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
    let mut missing_prefix = request(PublisherTarget::NativeWeb, "native_cwl_xapi_2_0/v1");
    missing_prefix.source_hash = "a".repeat(64);
    assert_eq!(
        evaluate_publication(missing_prefix),
        Err(AdmissionError::InvalidSourceHash)
    );

    let mut wrong_length = request(PublisherTarget::NativeWeb, "native_cwl_xapi_2_0/v1");
    wrong_length.source_hash = "sha256:not-a-digest".into();
    assert_eq!(
        evaluate_publication(wrong_length),
        Err(AdmissionError::InvalidSourceHash)
    );

    let mut non_hex = request(PublisherTarget::NativeWeb, "native_cwl_xapi_2_0/v1");
    non_hex.source_hash = format!("sha256:{}g", "a".repeat(63));
    assert_eq!(
        evaluate_publication(non_hex),
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
fn canonical_json_sorts_directly_constructed_incompatibility() {
    let later = BlockingFeature::new(
        "video_caption_missing",
        "component_b",
        "accessibility_evidence_missing",
    );
    let earlier = BlockingFeature::new(
        "audio_rights_missing",
        "component_a",
        "rights_evidence_missing",
    );
    let outcome = PublicationOutcome::Incompatible {
        metadata: metadata(),
        blocking_features: vec![later, earlier],
    };

    let json = outcome.canonical_json();
    let earlier_index = json
        .find("audio_rights_missing")
        .expect("earlier canonical blocker");
    let later_index = json
        .find("video_caption_missing")
        .expect("later canonical blocker");

    assert!(earlier_index < later_index);
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
fn canonical_json_escapes_all_json_control_classes() {
    let mut input = request(PublisherTarget::NativeWeb, "native_cwl_xapi_2_0/v1");
    input.content_release_id = "q\"\\\n\r\t\u{08}\u{0c}\u{01}é".into();

    let outcome = evaluate_publication(input).expect("valid escaped identity");
    assert!(outcome.canonical_json().contains(
        "\"content_release_id\":\"q\\\"\\\\\\n\\r\\t\\b\\f\\u0001é\""
    ));
}

#[test]
fn rejects_every_empty_required_identity_field() {
    let mut content_release = request(PublisherTarget::NativeWeb, "native_cwl_xapi_2_0/v1");
    content_release.content_release_id = " ".into();
    assert_eq!(
        evaluate_publication(content_release),
        Err(AdmissionError::EmptyRequiredField("content_release_id"))
    );

    let mut contract = request(PublisherTarget::NativeWeb, "native_cwl_xapi_2_0/v1");
    contract.publisher_contract_id = " ".into();
    assert_eq!(
        evaluate_publication(contract),
        Err(AdmissionError::EmptyRequiredField("publisher_contract_id"))
    );

    let mut version = request(PublisherTarget::NativeWeb, "native_cwl_xapi_2_0/v1");
    version.publisher_version = " ".into();
    assert_eq!(
        evaluate_publication(version),
        Err(AdmissionError::EmptyRequiredField("publisher_version"))
    );

    let mut revision = request(PublisherTarget::NativeWeb, "native_cwl_xapi_2_0/v1");
    revision.standard_revision = " ".into();
    assert_eq!(
        evaluate_publication(revision),
        Err(AdmissionError::EmptyRequiredField("standard_revision"))
    );

    let mut locale = request(PublisherTarget::NativeWeb, "native_cwl_xapi_2_0/v1");
    locale.locale_code = " ".into();
    assert_eq!(
        evaluate_publication(locale),
        Err(AdmissionError::EmptyRequiredField("locale_code"))
    );
}

#[test]
fn rejects_every_empty_blocking_feature_identity_field() {
    let mut feature_code = request(PublisherTarget::NativeWeb, "native_cwl_xapi_2_0/v1");
    feature_code.blocking_features = vec![BlockingFeature::new(" ", "component_1", "reason_1")];
    assert_eq!(
        evaluate_publication(feature_code),
        Err(AdmissionError::EmptyRequiredField(
            "blocking_feature.feature_code"
        ))
    );

    let mut source_reference = request(PublisherTarget::NativeWeb, "native_cwl_xapi_2_0/v1");
    source_reference.blocking_features = vec![BlockingFeature::new("feature_1", " ", "reason_1")];
    assert_eq!(
        evaluate_publication(source_reference),
        Err(AdmissionError::EmptyRequiredField(
            "blocking_feature.source_component_reference"
        ))
    );

    let mut reason_code = request(PublisherTarget::NativeWeb, "native_cwl_xapi_2_0/v1");
    reason_code.blocking_features = vec![BlockingFeature::new("feature_1", "component_1", " ")];
    assert_eq!(
        evaluate_publication(reason_code),
        Err(AdmissionError::EmptyRequiredField(
            "blocking_feature.reason_code"
        ))
    );
}
