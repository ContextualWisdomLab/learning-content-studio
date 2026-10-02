//! Durable append-only storage for native-web publication receipts.

use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use sha2::{Digest, Sha256};

use crate::NativeWebPublicationReceipt;

const RECORD_MAGIC: &[u8] = b"LCS_RECEIPT_V1\n";
const DIGEST_LENGTH: usize = 32;
const RECORD_HEADER_LENGTH: usize = RECORD_MAGIC.len() + (DIGEST_LENGTH * 2);
static TEMP_FILE_SEQUENCE: AtomicU64 = AtomicU64::new(0);
type SyncDirectory = fn(&Path) -> io::Result<()>;

/// Outcome of one idempotent append-only receipt write.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PublicationReceiptWrite {
    /// A new immutable receipt was durably linked into the store.
    Stored,
    /// The exact canonical receipt bytes were already present.
    AlreadyPresent,
}

/// Fail-closed durable receipt-store errors.
#[derive(Debug)]
pub enum PublicationReceiptStoreError {
    /// The filesystem rejected an operation required for durable storage.
    Io(io::Error),
    /// A persisted record failed its version, identity, or payload digest check.
    CorruptRecord,
    /// The same release and publisher contract already identify different bytes.
    ConflictingReceipt,
}

impl From<io::Error> for PublicationReceiptStoreError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

/// Filesystem-backed append-only repository for exact canonical receipt bytes.
#[derive(Clone, Debug)]
pub struct FilePublicationReceiptStore {
    directory: PathBuf,
}

impl FilePublicationReceiptStore {
    /// Opens or creates an isolated receipt directory.
    ///
    /// Orphaned hidden temporary files are ignored. Only atomically linked
    /// `*.receipt` records are visible to readers.
    ///
    /// # Errors
    ///
    /// Returns [`PublicationReceiptStoreError::Io`] when the directory cannot
    /// be created.
    pub fn open(directory: impl AsRef<Path>) -> Result<Self, PublicationReceiptStoreError> {
        Self::open_with(directory.as_ref(), sync_directory)
    }

    fn open_with(
        directory: &Path,
        sync_parent_directory: SyncDirectory,
    ) -> Result<Self, PublicationReceiptStoreError> {
        match fs::create_dir(directory) {
            Ok(()) => {}
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists && directory.is_dir() => {}
            Err(error) => return Err(error.into()),
        }
        let parent = directory
            .parent()
            .filter(|path| !path.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new("."));
        sync_parent_directory(parent)?;
        Ok(Self {
            directory: directory.to_path_buf(),
        })
    }

    /// Persists exact canonical receipt bytes once for a release/contract identity.
    ///
    /// An identical replay is idempotent. A replay with different canonical
    /// bytes fails closed and cannot replace the existing receipt.
    ///
    /// # Errors
    ///
    /// Returns an I/O, corruption, or conflicting-receipt error without
    /// replacing previously committed evidence.
    pub fn persist(
        &self,
        receipt: &NativeWebPublicationReceipt,
    ) -> Result<PublicationReceiptWrite, PublicationReceiptStoreError> {
        let identity = receipt_identity(receipt);
        let canonical_bytes = receipt.canonical_json().into_bytes();
        self.persist_canonical(
            &identity,
            &canonical_bytes,
            write_synced_temp,
            link_record,
            sync_directory,
        )
    }

    fn persist_canonical(
        &self,
        identity: &[u8],
        canonical_bytes: &[u8],
        write_temp: fn(&Path, &[u8]) -> io::Result<()>,
        link_record: fn(&Path, &Path) -> io::Result<()>,
        sync_store_directory: fn(&Path) -> io::Result<()>,
    ) -> Result<PublicationReceiptWrite, PublicationReceiptStoreError> {
        let record_path = self.record_path(identity);

        let record_bytes = encode_record(identity, canonical_bytes);
        let temp_path = loop {
            let sequence = TEMP_FILE_SEQUENCE.fetch_add(1, Ordering::Relaxed);
            let candidate = self.directory.join(format!(
                ".publication-receipt-{}-{sequence}.tmp",
                std::process::id()
            ));
            match write_temp(&candidate, &record_bytes) {
                Ok(()) => break candidate,
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {}
                Err(error) => return Err(error.into()),
            }
        };

        match link_record(&temp_path, &record_path) {
            Ok(()) => {
                let _ = fs::remove_file(&temp_path);
                sync_store_directory(&self.directory)?;
                Ok(PublicationReceiptWrite::Stored)
            }
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
                let _ = fs::remove_file(&temp_path);
                let existing = fs::read(&record_path)?;
                let existing = decode_record(&existing, identity)?;
                if existing == canonical_bytes {
                    sync_store_directory(&self.directory)?;
                    Ok(PublicationReceiptWrite::AlreadyPresent)
                } else {
                    Err(PublicationReceiptStoreError::ConflictingReceipt)
                }
            }
            Err(error) => {
                let _ = fs::remove_file(&temp_path);
                Err(error.into())
            }
        }
    }

    /// Loads exact canonical bytes for a release and publisher contract.
    ///
    /// # Errors
    ///
    /// Returns an I/O error or fails closed when the stored record is corrupt
    /// or belongs to a different identity.
    pub fn load(
        &self,
        content_release_id: &str,
        publisher_contract_id: &str,
    ) -> Result<Option<Vec<u8>>, PublicationReceiptStoreError> {
        let identity = joined_identity(content_release_id, publisher_contract_id);
        Self::load_record(&self.record_path(&identity), &identity)
    }

    /// Exercises deterministic storage-failure paths in coverage builds.
    #[cfg(coverage)]
    #[doc(hidden)]
    pub fn persist_canonical_for_coverage(
        &self,
        identity: &[u8],
        canonical_bytes: &[u8],
        write_temp: fn(&Path, &[u8]) -> io::Result<()>,
        link_record: fn(&Path, &Path) -> io::Result<()>,
        sync_store_directory: fn(&Path) -> io::Result<()>,
    ) -> Result<PublicationReceiptWrite, PublicationReceiptStoreError> {
        self.persist_canonical(
            identity,
            canonical_bytes,
            write_temp,
            link_record,
            sync_store_directory,
        )
    }

    /// Exercises deterministic store-creation failure paths in coverage builds.
    #[cfg(coverage)]
    #[doc(hidden)]
    pub fn open_with_sync_for_coverage(
        directory: &Path,
        sync_parent_directory: fn(&Path) -> io::Result<()>,
    ) -> Result<Self, PublicationReceiptStoreError> {
        Self::open_with(directory, sync_parent_directory)
    }

    fn record_path(&self, identity: &[u8]) -> PathBuf {
        self.directory
            .join(format!("{}.receipt", digest_hex(identity)))
    }

    fn load_record(
        path: &Path,
        identity: &[u8],
    ) -> Result<Option<Vec<u8>>, PublicationReceiptStoreError> {
        match fs::read(path) {
            Ok(record) => decode_record(&record, identity).map(Some),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(error) => Err(error.into()),
        }
    }
}

fn write_synced_temp(path: &Path, record_bytes: &[u8]) -> io::Result<()> {
    let mut temp_file = OpenOptions::new().write(true).create_new(true).open(path)?;
    temp_file
        .write_all(record_bytes)
        .and_then(|()| temp_file.sync_all())
}

fn sync_directory(path: &Path) -> io::Result<()> {
    File::open(path).and_then(|directory| directory.sync_all())
}

fn link_record(temp_path: &Path, record_path: &Path) -> io::Result<()> {
    fs::hard_link(temp_path, record_path)
}

fn receipt_identity(receipt: &NativeWebPublicationReceipt) -> Vec<u8> {
    joined_identity(
        receipt.metadata.content_release_id(),
        receipt.metadata.publisher_contract_id(),
    )
}

fn joined_identity(content_release_id: &str, publisher_contract_id: &str) -> Vec<u8> {
    let mut identity =
        Vec::with_capacity(content_release_id.len() + 1 + publisher_contract_id.len());
    identity.extend_from_slice(content_release_id.as_bytes());
    identity.push(0);
    identity.extend_from_slice(publisher_contract_id.as_bytes());
    identity
}

fn encode_record(identity: &[u8], canonical_bytes: &[u8]) -> Vec<u8> {
    let identity_digest = Sha256::digest(identity);
    let payload_digest = Sha256::digest(canonical_bytes);
    let mut record = Vec::with_capacity(RECORD_HEADER_LENGTH + canonical_bytes.len());
    record.extend_from_slice(RECORD_MAGIC);
    record.extend_from_slice(&identity_digest);
    record.extend_from_slice(&payload_digest);
    record.extend_from_slice(canonical_bytes);
    record
}

fn decode_record(record: &[u8], identity: &[u8]) -> Result<Vec<u8>, PublicationReceiptStoreError> {
    if record.len() < RECORD_HEADER_LENGTH || !record.starts_with(RECORD_MAGIC) {
        return Err(PublicationReceiptStoreError::CorruptRecord);
    }
    let identity_start = RECORD_MAGIC.len();
    let payload_digest_start = identity_start + DIGEST_LENGTH;
    let payload_start = payload_digest_start + DIGEST_LENGTH;
    let identity_digest = Sha256::digest(identity);
    let payload = &record[payload_start..];
    let payload_digest = Sha256::digest(payload);
    if record[identity_start..payload_digest_start] != identity_digest[..]
        || record[payload_digest_start..payload_start] != payload_digest[..]
    {
        return Err(PublicationReceiptStoreError::CorruptRecord);
    }
    Ok(payload.to_vec())
}

fn digest_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let digest = Sha256::digest(bytes);
    let mut output = String::with_capacity(digest.len() * 2);
    for byte in digest {
        output.push(char::from(HEX[usize::from(byte >> 4)]));
        output.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    output
}
