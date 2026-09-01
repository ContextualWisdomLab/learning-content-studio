//! Regression and edge-case contract tests for native-web publication finalization.

use learning_content_studio::{
    BlockingFeature, NativeWebPublicationError, PublicationRequest, PublisherTarget,
    evaluate_publication, finalize_native_web_publication,
};

const RELEASE_HASH: &str =
    "sha256:ff7a5e6429d2c8511521e4abf41cd54a3e525ef4a1f24f8d1c67ede9d17874dd";
const ARTIFACT_HASH: &str =
    "sha256:4659fc0570122b0e0aa14f4ff7c261b1fe51795a01ba79963f462ebf40d7520d";
const MANIFEST_HASH: &str =
    "sha256:a3400ac8544192ba8084bfff406a1872432f3ec6880d2dad53a2dc8a4ac31442";

const RELEASE_BYTES: &[u8] = b"release bytes";
const ARTIFACT_BYTES: &[u8] = b"artifact bytes";
const MANIFEST_BYTES: &[u8] = b"{\"files\":[\"index.html\"]}\n";

fn request(target: PublisherTarget, contract_id: &str, source_hash: &str) -> PublicationRequest {
    PublicationRequest {
        content_release_id: "content_release_01".into(),
        source_hash: source_hash.into(),
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
fn finalizes_byte_identical_native_web_receipts() {
    let outcome = evaluate_publication(request(
        PublisherTarget::NativeWeb,
        "native_cwl_xapi_2_0/v1",
        RELEASE_HASH,
    ))
    .expect("approved native release");

    let first = finalize_native_web_publication(
        &outcome,
        RELEASE_BYTES,
        ARTIFACT_BYTES,
        MANIFEST_BYTES,
        &["validation_receipt_b", "validation_receipt_a"],
    )
    .expect("valid native publication");
    let second = finalize_native_web_publication(
        &outcome,
        RELEASE_BYTES,
        ARTIFACT_BYTES,
        MANIFEST_BYTES,
        &["validation_receipt_a", "validation_receipt_b"],
    )
    .expect("valid native publication");

    assert_eq!(first, second);
    assert_eq!(first.metadata().content_release_id(), "content_release_01");
    assert_eq!(first.metadata().source_hash(), RELEASE_HASH);
    assert_eq!(first.artifact_hash(), ARTIFACT_HASH);
    assert_eq!(first.build_manifest_hash(), MANIFEST_HASH);
    assert_eq!(
        first.validation_receipt_ids(),
        &["validation_receipt_a", "validation_receipt_b"]
    );
    assert_eq!(first.canonical_json(), second.canonical_json());
    assert!(
        first
            .canonical_json()
            .contains("\"publisher_contract_id\":\"native_cwl_xapi_2_0/v1\"")
    );
    assert!(
        first
            .canonical_json()
            .contains(&format!("\"artifact_hash\":\"{ARTIFACT_HASH}\""))
    );
    assert!(
        first
            .canonical_json()
            .contains(&format!("\"build_manifest_hash\":\"{MANIFEST_HASH}\""))
    );
}

#[test]
fn source_bytes_must_match_admitted_release_identity() {
    let outcome = evaluate_publication(request(
        PublisherTarget::NativeWeb,
        "native_cwl_xapi_2_0/v1",
        RELEASE_HASH,
    ))
    .expect("approved native release");

    assert_eq!(
        finalize_native_web_publication(
            &outcome,
            b"different release bytes",
            ARTIFACT_BYTES,
            MANIFEST_BYTES,
            &[],
        ),
        Err(NativeWebPublicationError::SourceHashMismatch)
    );
}

#[test]
fn source_hash_case_does_not_change_digest_identity() {
    let uppercase_hash = RELEASE_HASH.to_ascii_uppercase().replacen("SHA256:", "sha256:", 1);
    let outcome = evaluate_publication(request(
        PublisherTarget::NativeWeb,
        "native_cwl_xapi_2_0/v1",
        &uppercase_hash,
    ))
    .expect("uppercase hexadecimal is valid admission identity");

    let receipt = finalize_native_web_publication(
        &outcome,
        RELEASE_BYTES,
        ARTIFACT_BYTES,
        MANIFEST_BYTES,
        &[],
    )
    .expect("digest comparison is case-insensitive");

    assert_eq!(receipt.metadata().source_hash(), uppercase_hash);
    assert_eq!(receipt.artifact_hash(), ARTIFACT_HASH);
}

#[test]
fn incompatible_admission_cannot_be_finalized() {
    let mut input = request(
        PublisherTarget::NativeWeb,
        "native_cwl_xapi_2_0/v1",
        RELEASE_HASH,
    );
    input.blocking_features = vec![BlockingFeature::new(
        "unsupported_feature",
        "component_1",
        "semantic_loss_required",
    )];
    let outcome = evaluate_publication(input).expect("trusted incompatible outcome");

    assert_eq!(
        finalize_native_web_publication(
            &outcome,
            RELEASE_BYTES,
            ARTIFACT_BYTES,
            MANIFEST_BYTES,
            &[],
        ),
        Err(NativeWebPublicationError::PublicationIncompatible)
    );
}

#[test]
fn cmi5_contract_cannot_cross_native_web_finalizer() {
    let outcome = evaluate_publication(request(
        PublisherTarget::Cmi5Quartz,
        "cmi5_quartz_xapi_1_0_3/v1",
        RELEASE_HASH,
    ))
    .expect("approved cmi5 release");

    assert_eq!(
        finalize_native_web_publication(
            &outcome,
            RELEASE_BYTES,
            ARTIFACT_BYTES,
            MANIFEST_BYTES,
            &[],
        ),
        Err(NativeWebPublicationError::WrongPublisherContract)
    );
}

#[test]
fn empty_emitted_bytes_fail_closed() {
    let outcome = evaluate_publication(request(
        PublisherTarget::NativeWeb,
        "native_cwl_xapi_2_0/v1",
        RELEASE_HASH,
    ))
    .expect("approved native release");

    assert_eq!(
        finalize_native_web_publication(&outcome, RELEASE_BYTES, &[], MANIFEST_BYTES, &[]),
        Err(NativeWebPublicationError::EmptyArtifact)
    );
    assert_eq!(
        finalize_native_web_publication(&outcome, RELEASE_BYTES, ARTIFACT_BYTES, &[], &[]),
        Err(NativeWebPublicationError::EmptyBuildManifest)
    );
}

#[test]
fn validation_receipt_identity_is_canonical_and_nonempty() {
    let outcome = evaluate_publication(request(
        PublisherTarget::NativeWeb,
        "native_cwl_xapi_2_0/v1",
        RELEASE_HASH,
    ))
    .expect("approved native release");

    assert_eq!(
        finalize_native_web_publication(
            &outcome,
            RELEASE_BYTES,
            ARTIFACT_BYTES,
            MANIFEST_BYTES,
            &["validation_receipt_a", "  "],
        ),
        Err(NativeWebPublicationError::EmptyValidationReceiptId)
    );
    assert_eq!(
        finalize_native_web_publication(
            &outcome,
            RELEASE_BYTES,
            ARTIFACT_BYTES,
            MANIFEST_BYTES,
            &["validation_receipt_a", "validation_receipt_a"],
        ),
        Err(NativeWebPublicationError::DuplicateValidationReceiptId)
    );
}
