//! Trust-boundary regressions for publication admission authority.

use learning_content_studio::{
    AdmissionError, BlockingFeature, PublicationRequest, PublicationStatus, PublisherTarget,
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

fn request(target: PublisherTarget) -> PublicationRequest {
    PublicationRequest::new("content_release_01", target)
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

fn compatibility(
    target: PublisherTarget,
    blockers: Vec<BlockingFeature>,
) -> TargetCompatibilityEvidence {
    let contract = match target {
        PublisherTarget::NativeWeb => "native_cwl_xapi_2_0/v1",
        PublisherTarget::Cmi5Quartz => "cmi5_quartz_xapi_1_0_3/v1",
    };
    TargetCompatibilityEvidence::new(
        target,
        contract,
        "1.0.0",
        "2026-08",
        "target_validation_receipt_01",
        blockers,
    )
}

#[test]
fn caller_cannot_assert_approval_or_omit_authoritative_blockers() {
    let release_authority = FixedReleaseAuthority {
        evidence: approved_release(),
    };
    let compatibility_authority = FixedCompatibilityAuthority {
        evidence: compatibility(
            PublisherTarget::NativeWeb,
            vec![BlockingFeature::new(
                "video_caption_missing",
                "component_7",
                "accessibility_evidence_missing",
            )],
        ),
    };

    let outcome = evaluate_publication(
        request(PublisherTarget::NativeWeb),
        &release_authority,
        &compatibility_authority,
    )
    .expect("authority-backed incompatible outcome");

    assert_eq!(outcome.status(), PublicationStatus::Incompatible);
    assert_eq!(outcome.blocking_features().len(), 1);
}

#[test]
fn authority_owned_unapproved_release_fails_closed() {
    let release_authority = FixedReleaseAuthority {
        evidence: ReleaseAuthorityEvidence::new(
            "content_release_01",
            &format!("sha256:{}", "a".repeat(64)),
            "en-US",
            false,
            "release_approval_receipt_01",
        ),
    };
    let compatibility_authority = FixedCompatibilityAuthority {
        evidence: compatibility(PublisherTarget::NativeWeb, Vec::new()),
    };

    assert_eq!(
        evaluate_publication(
            request(PublisherTarget::NativeWeb),
            &release_authority,
            &compatibility_authority,
        ),
        Err(AdmissionError::ReleaseNotApproved)
    );
}

#[test]
fn release_authority_cannot_swap_release_identity() {
    let release_authority = FixedReleaseAuthority {
        evidence: ReleaseAuthorityEvidence::new(
            "content_release_other",
            &format!("sha256:{}", "a".repeat(64)),
            "en-US",
            true,
            "release_approval_receipt_01",
        ),
    };
    let compatibility_authority = FixedCompatibilityAuthority {
        evidence: compatibility(PublisherTarget::NativeWeb, Vec::new()),
    };

    assert_eq!(
        evaluate_publication(
            request(PublisherTarget::NativeWeb),
            &release_authority,
            &compatibility_authority,
        ),
        Err(AdmissionError::ReleaseAuthorityMismatch)
    );
}

#[test]
fn compatibility_authority_cannot_swap_target() {
    let release_authority = FixedReleaseAuthority {
        evidence: approved_release(),
    };
    let compatibility_authority = FixedCompatibilityAuthority {
        evidence: compatibility(PublisherTarget::Cmi5Quartz, Vec::new()),
    };

    assert_eq!(
        evaluate_publication(
            request(PublisherTarget::NativeWeb),
            &release_authority,
            &compatibility_authority,
        ),
        Err(AdmissionError::CompatibilityAuthorityMismatch)
    );
}

#[test]
fn missing_authority_evidence_fails_closed() {
    struct MissingRelease;
    impl ReleaseAuthorityPort for MissingRelease {
        fn release_evidence(&self, _content_release_id: &str) -> Option<ReleaseAuthorityEvidence> {
            None
        }
    }

    struct MissingCompatibility;
    impl TargetCompatibilityPort for MissingCompatibility {
        fn compatibility_evidence(
            &self,
            _release: &ReleaseAuthorityEvidence,
            _target: PublisherTarget,
        ) -> Option<TargetCompatibilityEvidence> {
            None
        }
    }

    let compatibility_authority = FixedCompatibilityAuthority {
        evidence: compatibility(PublisherTarget::NativeWeb, Vec::new()),
    };
    assert_eq!(
        evaluate_publication(
            request(PublisherTarget::NativeWeb),
            &MissingRelease,
            &compatibility_authority,
        ),
        Err(AdmissionError::ReleaseEvidenceUnavailable)
    );

    let release_authority = FixedReleaseAuthority {
        evidence: approved_release(),
    };
    assert_eq!(
        evaluate_publication(
            request(PublisherTarget::NativeWeb),
            &release_authority,
            &MissingCompatibility,
        ),
        Err(AdmissionError::CompatibilityEvidenceUnavailable)
    );
}
