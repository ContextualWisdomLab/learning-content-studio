//! Regressions for exact immutable-release binding of target compatibility evidence.

use learning_content_studio::{
    AdmissionError, CompatibilityReleaseIdentity, PublicationRequest, PublisherTarget,
    ReleaseAuthorityEvidence, ReleaseAuthorityPort, TargetCompatibilityEvidence,
    TargetCompatibilityPort, evaluate_publication,
};

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

fn approved_release() -> ReleaseAuthorityEvidence {
    ReleaseAuthorityEvidence::new(
        "content_release_01",
        &format!("sha256:{}", "a".repeat(64)),
        "en-US",
        true,
        "release_approval_receipt_01",
    )
}

fn target_evidence(content_release_id: &str, source_hash: &str) -> TargetCompatibilityEvidence {
    TargetCompatibilityEvidence::new(
        CompatibilityReleaseIdentity::new(content_release_id, source_hash),
        PublisherTarget::NativeWeb,
        "native_cwl_xapi_2_0/v1",
        "1.0.0",
        "2026-08",
        "target_validation_receipt_01",
        Vec::new(),
    )
}

fn evaluate(
    evidence: TargetCompatibilityEvidence,
) -> Result<learning_content_studio::PublicationOutcome, AdmissionError> {
    let release_authority = FixedReleaseAuthority {
        evidence: approved_release(),
    };
    let compatibility_authority = FixedCompatibilityAuthority { evidence };
    evaluate_publication(
        PublicationRequest::new("content_release_01", PublisherTarget::NativeWeb),
        &release_authority,
        &compatibility_authority,
    )
}

#[test]
fn compatibility_evidence_for_another_release_fails_closed() {
    assert_eq!(
        evaluate(target_evidence(
            "content_release_other",
            &format!("sha256:{}", "a".repeat(64)),
        )),
        Err(AdmissionError::CompatibilityReleaseMismatch)
    );
}

#[test]
fn compatibility_evidence_for_another_source_hash_fails_closed() {
    assert_eq!(
        evaluate(target_evidence(
            "content_release_01",
            &format!("sha256:{}", "b".repeat(64)),
        )),
        Err(AdmissionError::CompatibilitySourceMismatch)
    );
}
