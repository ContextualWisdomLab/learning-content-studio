//! Regression and edge-case contract tests for native-web publication finalization.

use learning_content_studio::{
    BlockingFeature, CompatibilityReleaseIdentity, NativeWebPublicationError, PublicationRequest,
    PublisherTarget, ReleaseAuthorityEvidence, ReleaseAuthorityPort, TargetCompatibilityEvidence,
    TargetCompatibilityPort, evaluate_publication, finalize_native_web_publication,
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

#[derive(Clone)]
struct FixedReleaseAuthority {
    evidence: ReleaseAuthorityEvidence,
}

impl ReleaseAuthorityPort for FixedReleaseAuthority {
    fn release_evidence(&self, _content_release_id: &str) -> Option<ReleaseAuthorityEvidence> {
        Some(self.evidence.clone())
    }
}

#[derive(Clone)]
struct FixedCompatibilityAuthority {
    evidence: TargetCompatibilityEvidence,
}

impl TargetCompatibilityPort for FixedCompatibilityAuthority {
    fn compatibility_evidence(
        &self,
        _release: &ReleaseAuthorityEvidence,
        _target: PublisherTarget,
    ) -> Option<TargetCompatibilityEvidence> {
        Some(self.evidence.clone())
    }
}

fn contract(target: PublisherTarget) -> &'static str {
    match target {
        PublisherTarget::NativeWeb => "native_cwl_xapi_2_0/v1",
        PublisherTarget::Cmi5Quartz => "cmi5_quartz_xapi_1_0_3/v1",
    }
}

fn admitted_outcome(
    target: PublisherTarget,
    source_hash: &str,
    blockers: Vec<BlockingFeature>,
) -> learning_content_studio::PublicationOutcome {
    let release_authority = FixedReleaseAuthority {
        evidence: ReleaseAuthorityEvidence::new(
            "content_release_01",
            source_hash,
            "en-US",
            true,
            "release_approval_receipt_01",
        ),
    };
    let compatibility_authority = FixedCompatibilityAuthority {
        evidence: TargetCompatibilityEvidence::new(
            CompatibilityReleaseIdentity::new("content_release_01", source_hash),
            target,
            contract(target),
            "1.0.0",
            "2026-08",
            "target_validation_receipt_01",
            blockers,
        ),
    };
    evaluate_publication(
        PublicationRequest::new("content_release_01", target),
        &release_authority,
        &compatibility_authority,
    )
    .expect("authority-backed admission")
}

#[test]
fn finalizes_byte_identical_native_web_receipts() {
    let outcome = admitted_outcome(PublisherTarget::NativeWeb, RELEASE_HASH, Vec::new());

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
    assert_eq!(
        first.metadata().release_approval_evidence_id(),
        "release_approval_receipt_01"
    );
    assert_eq!(
        first.metadata().target_validation_evidence_id(),
        "target_validation_receipt_01"
    );
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
            .contains("\"release_approval_evidence_id\":\"release_approval_receipt_01\"")
    );
    assert!(
        first
            .canonical_json()
            .contains("\"target_validation_evidence_id\":\"target_validation_receipt_01\"")
    );
}

#[test]
fn source_bytes_must_match_admitted_release_identity() {
    let outcome = admitted_outcome(PublisherTarget::NativeWeb, RELEASE_HASH, Vec::new());
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
fn equivalent_source_hash_case_produces_identical_receipt_evidence() {
    let uppercase_hash = RELEASE_HASH
        .to_ascii_uppercase()
        .replacen("SHA256:", "sha256:", 1);
    let uppercase_outcome =
        admitted_outcome(PublisherTarget::NativeWeb, &uppercase_hash, Vec::new());
    let lowercase_outcome = admitted_outcome(PublisherTarget::NativeWeb, RELEASE_HASH, Vec::new());

    let uppercase_receipt = finalize_native_web_publication(
        &uppercase_outcome,
        RELEASE_BYTES,
        ARTIFACT_BYTES,
        MANIFEST_BYTES,
        &[],
    )
    .expect("uppercase digest finalizes");
    let lowercase_receipt = finalize_native_web_publication(
        &lowercase_outcome,
        RELEASE_BYTES,
        ARTIFACT_BYTES,
        MANIFEST_BYTES,
        &[],
    )
    .expect("lowercase digest finalizes");

    assert_eq!(uppercase_outcome.metadata().source_hash(), uppercase_hash);
    assert_eq!(uppercase_receipt.metadata().source_hash(), RELEASE_HASH);
    assert_eq!(uppercase_receipt, lowercase_receipt);
    assert_eq!(
        uppercase_receipt.canonical_json(),
        lowercase_receipt.canonical_json()
    );
}

#[test]
fn incompatible_admission_cannot_be_finalized() {
    let outcome = admitted_outcome(
        PublisherTarget::NativeWeb,
        RELEASE_HASH,
        vec![BlockingFeature::new(
            "unsupported_feature",
            "component_1",
            "semantic_loss_required",
        )],
    );
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
    let outcome = admitted_outcome(PublisherTarget::Cmi5Quartz, RELEASE_HASH, Vec::new());
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
    let outcome = admitted_outcome(PublisherTarget::NativeWeb, RELEASE_HASH, Vec::new());
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
    let outcome = admitted_outcome(PublisherTarget::NativeWeb, RELEASE_HASH, Vec::new());
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
