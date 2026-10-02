//! Durable native-web publication-receipt repository regressions.

use std::fs;
#[cfg(coverage)]
use std::io;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use learning_content_studio::{
    CompatibilityReleaseIdentity, FilePublicationReceiptStore, PublicationReceiptStoreError,
    PublicationReceiptWrite, PublicationRequest, PublisherTarget, ReleaseAuthorityEvidence,
    ReleaseAuthorityPort, TargetCompatibilityEvidence, TargetCompatibilityPort,
    evaluate_publication, finalize_native_web_publication,
};
use sha2::{Digest, Sha256};

const RELEASE_HASH: &str =
    "sha256:ff7a5e6429d2c8511521e4abf41cd54a3e525ef4a1f24f8d1c67ede9d17874dd";
const RELEASE_BYTES: &[u8] = b"release bytes";
const HEX: &[u8; 16] = b"0123456789abcdef";
static TEST_DIRECTORY_SEQUENCE: AtomicU64 = AtomicU64::new(0);
#[cfg(coverage)]
static COVERAGE_TEMP_WRITE_ATTEMPTS: AtomicU64 = AtomicU64::new(0);

struct TestDirectory(PathBuf);

impl TestDirectory {
    fn new() -> Self {
        let sequence = TEST_DIRECTORY_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "learning-content-studio-receipts-{}-{sequence}",
            std::process::id()
        ));
        fs::create_dir(&path).expect("create isolated receipt directory");
        Self(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).expect("remove isolated receipt directory");
    }
}

#[derive(Clone)]
struct FixedReleaseAuthority;

impl ReleaseAuthorityPort for FixedReleaseAuthority {
    fn release_evidence(&self, content_release_id: &str) -> Option<ReleaseAuthorityEvidence> {
        Some(ReleaseAuthorityEvidence::new(
            content_release_id,
            RELEASE_HASH,
            "en-US",
            true,
            "release_approval_receipt_01",
        ))
    }
}

#[derive(Clone)]
struct FixedCompatibilityAuthority;

impl TargetCompatibilityPort for FixedCompatibilityAuthority {
    fn compatibility_evidence(
        &self,
        release: &ReleaseAuthorityEvidence,
        target: PublisherTarget,
    ) -> Option<TargetCompatibilityEvidence> {
        Some(TargetCompatibilityEvidence::new(
            CompatibilityReleaseIdentity::new(release.content_release_id(), release.source_hash()),
            target,
            "native_cwl_xapi_2_0/v1",
            "1.0.0",
            "2026-08",
            "target_validation_receipt_01",
            Vec::new(),
        ))
    }
}

fn receipt(artifact_bytes: &[u8]) -> learning_content_studio::NativeWebPublicationReceipt {
    receipt_with_validation_ids(artifact_bytes, &["validation_receipt_01"])
}

fn receipt_with_validation_ids(
    artifact_bytes: &[u8],
    validation_receipt_ids: &[&str],
) -> learning_content_studio::NativeWebPublicationReceipt {
    let outcome = evaluate_publication(
        PublicationRequest::new("content_release_01", PublisherTarget::NativeWeb),
        &FixedReleaseAuthority,
        &FixedCompatibilityAuthority,
    )
    .expect("authority-backed admission");
    finalize_native_web_publication(
        &outcome,
        RELEASE_BYTES,
        artifact_bytes,
        b"{\"files\":[\"index.html\"]}\n",
        validation_receipt_ids,
    )
    .expect("byte-finalized publication")
}

fn receipt_path(
    directory: &Path,
    content_release_id: &str,
    publisher_contract_id: &str,
) -> PathBuf {
    let mut identity = Vec::new();
    identity.extend_from_slice(content_release_id.as_bytes());
    identity.push(0);
    identity.extend_from_slice(publisher_contract_id.as_bytes());
    let digest = Sha256::digest(identity);
    let mut name = String::with_capacity((digest.len() * 2) + ".receipt".len());
    for byte in digest {
        name.push(char::from(HEX[usize::from(byte >> 4)]));
        name.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    name.push_str(".receipt");
    directory.join(name)
}

#[cfg(coverage)]
fn coverage_identity() -> Vec<u8> {
    let mut identity = Vec::from("content_release_01".as_bytes());
    identity.push(0);
    identity.extend_from_slice(b"native_cwl_xapi_2_0/v1");
    identity
}

#[cfg(coverage)]
fn coverage_write_temp(path: &Path, bytes: &[u8]) -> io::Result<()> {
    fs::write(path, bytes)
}

#[cfg(coverage)]
fn coverage_collide_once_then_write(path: &Path, bytes: &[u8]) -> io::Result<()> {
    if COVERAGE_TEMP_WRITE_ATTEMPTS.fetch_add(1, Ordering::SeqCst) == 0 {
        Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "injected occupied temp candidate",
        ))
    } else {
        fs::write(path, bytes)
    }
}

#[test]
fn receipt_survives_store_reopen_as_exact_canonical_bytes() {
    let directory = TestDirectory::new();
    let receipt = receipt(b"artifact bytes");
    let canonical_bytes = receipt.canonical_json().into_bytes();

    let store = FilePublicationReceiptStore::open(directory.path()).expect("open store");
    assert_eq!(
        store.persist(&receipt).expect("persist receipt"),
        PublicationReceiptWrite::Stored
    );
    drop(store);

    let reopened = FilePublicationReceiptStore::open(directory.path()).expect("reopen store");
    assert_eq!(
        reopened
            .load("content_release_01", "native_cwl_xapi_2_0/v1")
            .expect("load receipt"),
        Some(canonical_bytes)
    );
}

#[test]
fn missing_receipt_is_reported_as_absent() {
    let directory = TestDirectory::new();
    let store = FilePublicationReceiptStore::open(directory.path()).expect("open store");

    assert_eq!(
        store
            .load("content_release_missing", "native_cwl_xapi_2_0/v1")
            .expect("load missing receipt"),
        None
    );
}

#[test]
fn identical_replay_is_idempotent_without_a_second_record() {
    let directory = TestDirectory::new();
    let receipt = receipt(b"artifact bytes");
    let store = FilePublicationReceiptStore::open(directory.path()).expect("open store");

    assert_eq!(
        store.persist(&receipt).expect("first write"),
        PublicationReceiptWrite::Stored
    );
    assert_eq!(
        store.persist(&receipt).expect("idempotent replay"),
        PublicationReceiptWrite::AlreadyPresent
    );
    assert_eq!(
        fs::read_dir(directory.path())
            .expect("list store")
            .filter_map(Result::ok)
            .filter(|entry| entry
                .path()
                .extension()
                .is_some_and(|value| value == "receipt"))
            .count(),
        1
    );
}

#[test]
fn conflicting_replay_for_the_same_release_and_contract_fails_closed() {
    let directory = TestDirectory::new();
    let first = receipt(b"artifact bytes");
    let conflicting = receipt(b"different artifact bytes");
    let store = FilePublicationReceiptStore::open(directory.path()).expect("open store");

    assert_eq!(
        store.persist(&first).expect("first write"),
        PublicationReceiptWrite::Stored
    );
    assert!(matches!(
        store.persist(&conflicting),
        Err(PublicationReceiptStoreError::ConflictingReceipt)
    ));
    assert_eq!(
        store
            .load("content_release_01", "native_cwl_xapi_2_0/v1")
            .expect("load original"),
        Some(first.canonical_json().into_bytes())
    );
}

#[test]
fn orphaned_interrupted_write_is_ignored_during_recovery() {
    let directory = TestDirectory::new();
    fs::write(
        directory
            .path()
            .join(".publication-receipt-interrupted.tmp"),
        b"partial record",
    )
    .expect("write interrupted temp record");
    let receipt = receipt(b"artifact bytes");

    let store = FilePublicationReceiptStore::open(directory.path()).expect("reopen store");
    assert_eq!(
        store.persist(&receipt).expect("recover with valid write"),
        PublicationReceiptWrite::Stored
    );
    assert_eq!(
        store
            .load("content_release_01", "native_cwl_xapi_2_0/v1")
            .expect("load recovered receipt"),
        Some(receipt.canonical_json().into_bytes())
    );
}

#[test]
fn missing_store_directory_reports_io_without_committing() {
    let directory = TestDirectory::new();
    let store = FilePublicationReceiptStore::open(directory.path()).expect("open store");
    fs::remove_dir(directory.path()).expect("remove empty store");

    assert!(matches!(
        store.persist(&receipt(b"artifact bytes")),
        Err(PublicationReceiptStoreError::Io(_))
    ));
    fs::create_dir(directory.path()).expect("restore store for cleanup");
}

#[test]
fn open_fails_when_the_store_path_is_an_existing_file() {
    let directory = TestDirectory::new();
    let file_path = directory.path().join("not_a_directory");
    fs::write(&file_path, b"occupied").expect("write occupied path");

    assert!(matches!(
        FilePublicationReceiptStore::open(file_path),
        Err(PublicationReceiptStoreError::Io(_))
    ));
}

#[test]
fn open_durably_creates_one_store_leaf_under_an_existing_parent() {
    let parent = TestDirectory::new();
    let store_path = parent.path().join("receipt_store");

    FilePublicationReceiptStore::open(&store_path).expect("create store leaf");

    assert!(store_path.is_dir());
}

#[test]
fn open_rejects_a_store_whose_parent_is_not_provisioned() {
    let parent = TestDirectory::new();
    let store_path = parent.path().join("missing_parent/receipt_store");

    assert!(matches!(
        FilePublicationReceiptStore::open(store_path),
        Err(PublicationReceiptStoreError::Io(_))
    ));
}

#[test]
fn open_syncs_the_current_directory_for_a_relative_store_leaf() {
    let sequence = TEST_DIRECTORY_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let store_path = PathBuf::from(format!(
        "learning-content-studio-relative-receipts-{}-{sequence}",
        std::process::id()
    ));

    FilePublicationReceiptStore::open(&store_path).expect("create relative store leaf");

    fs::remove_dir(store_path).expect("remove relative store leaf");
}

#[test]
fn unreadable_committed_record_reports_io_instead_of_absence() {
    let directory = TestDirectory::new();
    let record_path = receipt_path(
        directory.path(),
        "content_release_01",
        "native_cwl_xapi_2_0/v1",
    );
    fs::create_dir(&record_path).expect("create unreadable record path");
    let store = FilePublicationReceiptStore::open(directory.path()).expect("open store");

    assert!(matches!(
        store.persist(&receipt(b"artifact bytes")),
        Err(PublicationReceiptStoreError::Io(_))
    ));
    assert!(matches!(
        store.load("content_release_01", "native_cwl_xapi_2_0/v1"),
        Err(PublicationReceiptStoreError::Io(_))
    ));
}

#[test]
fn truncated_committed_record_fails_closed() {
    let directory = TestDirectory::new();
    let record_path = receipt_path(
        directory.path(),
        "content_release_01",
        "native_cwl_xapi_2_0/v1",
    );
    fs::write(&record_path, b"truncated").expect("write truncated record");
    let store = FilePublicationReceiptStore::open(directory.path()).expect("open store");

    assert!(matches!(
        store.persist(&receipt(b"artifact bytes")),
        Err(PublicationReceiptStoreError::CorruptRecord)
    ));
    assert!(matches!(
        store.load("content_release_01", "native_cwl_xapi_2_0/v1"),
        Err(PublicationReceiptStoreError::CorruptRecord)
    ));
}

#[test]
fn unknown_record_version_fails_closed() {
    let directory = TestDirectory::new();
    let receipt = receipt(b"artifact bytes");
    let store = FilePublicationReceiptStore::open(directory.path()).expect("open store");
    store.persist(&receipt).expect("persist receipt");
    let record_path = receipt_path(
        directory.path(),
        "content_release_01",
        "native_cwl_xapi_2_0/v1",
    );
    let mut record = fs::read(&record_path).expect("read record");
    record[0] ^= 1;
    fs::write(&record_path, record).expect("replace record magic");

    assert!(matches!(
        store.load("content_release_01", "native_cwl_xapi_2_0/v1"),
        Err(PublicationReceiptStoreError::CorruptRecord)
    ));
}

#[test]
fn payload_tampering_fails_closed() {
    let directory = TestDirectory::new();
    let receipt = receipt(b"artifact bytes");
    let store = FilePublicationReceiptStore::open(directory.path()).expect("open store");
    store.persist(&receipt).expect("persist receipt");
    let record_path = receipt_path(
        directory.path(),
        "content_release_01",
        "native_cwl_xapi_2_0/v1",
    );
    let mut record = fs::read(&record_path).expect("read record");
    *record.last_mut().expect("record payload") ^= 1;
    fs::write(&record_path, record).expect("tamper record");

    assert!(matches!(
        store.load("content_release_01", "native_cwl_xapi_2_0/v1"),
        Err(PublicationReceiptStoreError::CorruptRecord)
    ));
}

#[test]
fn record_copied_to_another_identity_fails_closed() {
    let directory = TestDirectory::new();
    let receipt = receipt(b"artifact bytes");
    let store = FilePublicationReceiptStore::open(directory.path()).expect("open store");
    store.persist(&receipt).expect("persist receipt");
    let source_path = receipt_path(
        directory.path(),
        "content_release_01",
        "native_cwl_xapi_2_0/v1",
    );
    let copied_path = receipt_path(
        directory.path(),
        "content_release_02",
        "native_cwl_xapi_2_0/v1",
    );
    fs::copy(source_path, copied_path).expect("copy record to mismatched identity");

    assert!(matches!(
        store.load("content_release_02", "native_cwl_xapi_2_0/v1"),
        Err(PublicationReceiptStoreError::CorruptRecord)
    ));
}

#[cfg(coverage)]
#[test]
fn occupied_temp_candidate_is_skipped_without_deletion() {
    let directory = TestDirectory::new();
    let store = FilePublicationReceiptStore::open(directory.path()).expect("open store");
    COVERAGE_TEMP_WRITE_ATTEMPTS.store(0, Ordering::SeqCst);

    assert_eq!(
        store
            .persist_canonical_for_coverage(
                &coverage_identity(),
                b"canonical receipt",
                coverage_collide_once_then_write,
                |temp_path, record_path| fs::hard_link(temp_path, record_path),
                |_| Ok(()),
            )
            .expect("retry after occupied temp candidate"),
        PublicationReceiptWrite::Stored
    );
    assert_eq!(COVERAGE_TEMP_WRITE_ATTEMPTS.load(Ordering::SeqCst), 2);
}

#[cfg(coverage)]
#[test]
fn injected_storage_failures_are_fail_closed_and_cleanup_temps() {
    let directory = TestDirectory::new();
    let store = FilePublicationReceiptStore::open(directory.path()).expect("open store");
    let identity = coverage_identity();

    assert!(matches!(
        store.persist_canonical_for_coverage(
            &identity,
            b"canonical receipt",
            |_, _| Err(io::Error::other("injected temp-write failure")),
            |_, _| Ok(()),
            |_| Ok(()),
        ),
        Err(PublicationReceiptStoreError::Io(_))
    ));
    assert!(matches!(
        store.persist_canonical_for_coverage(
            &identity,
            b"canonical receipt",
            coverage_write_temp,
            |_, _| Err(io::Error::other("injected link failure")),
            |_| Ok(()),
        ),
        Err(PublicationReceiptStoreError::Io(_))
    ));
    assert_eq!(
        fs::read_dir(directory.path())
            .expect("list store")
            .filter_map(Result::ok)
            .count(),
        0
    );
    assert!(matches!(
        store.persist_canonical_for_coverage(
            &identity,
            b"canonical receipt",
            coverage_write_temp,
            |temp_path, record_path| fs::hard_link(temp_path, record_path),
            |_| Err(io::Error::other("injected directory-sync failure")),
        ),
        Err(PublicationReceiptStoreError::Io(_))
    ));
}

#[cfg(coverage)]
#[test]
fn identical_replay_requires_a_successful_directory_sync() {
    let directory = TestDirectory::new();
    let receipt = receipt(b"artifact bytes");
    let canonical_bytes = receipt.canonical_json().into_bytes();
    let store = FilePublicationReceiptStore::open(directory.path()).expect("open store");
    store.persist(&receipt).expect("persist first receipt");

    assert!(matches!(
        store.persist_canonical_for_coverage(
            &coverage_identity(),
            &canonical_bytes,
            coverage_write_temp,
            |temp_path, record_path| fs::hard_link(temp_path, record_path),
            |_| Err(io::Error::other("injected replay-sync failure")),
        ),
        Err(PublicationReceiptStoreError::Io(_))
    ));
}

#[cfg(coverage)]
#[test]
fn newly_created_store_fails_when_parent_sync_fails() {
    let parent = TestDirectory::new();
    let store_path = parent.path().join("receipt_store_sync_failure");

    assert!(matches!(
        FilePublicationReceiptStore::open_with_sync_for_coverage(&store_path, |_| {
            Err(io::Error::other("injected parent-sync failure"))
        }),
        Err(PublicationReceiptStoreError::Io(_))
    ));
}
