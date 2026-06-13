//! Secure handling of raw key material.
//!
//! `SecretBytes` owns a heap buffer that is:
//!   * memory-locked (best effort) so the OS will not page it to swap/disk,
//!   * overwritten with zeros the moment it is dropped.
//!
//! This is the only place raw key bytes are stored. Everything else derives a
//! short-lived, zeroizing hex string from it on demand.

use zeroize::Zeroize;

/// A heap-allocated secret that is zeroized on drop and locked into RAM.
pub struct SecretBytes {
    bytes: Vec<u8>,
    /// Holds the page lock for `bytes`; dropping it unlocks the region.
    lock: Option<region::LockGuard>,
}

impl SecretBytes {
    /// Take ownership of `bytes`, attempting to lock the pages into RAM.
    ///
    /// Locking is best effort: on some platforms or under tight rlimits it may
    /// fail, in which case the data is still zeroized on drop but may be
    /// swappable. Callers can check [`SecretBytes::is_locked`].
    pub fn new(bytes: Vec<u8>) -> Self {
        let lock = if bytes.is_empty() {
            None
        } else {
            // The pointer/len describe the live allocation owned by `bytes`,
            // which is not reallocated for the lifetime of `self` (we never
            // grow it). The guard is dropped before `bytes` in `Drop`.
            region::lock(bytes.as_ptr(), bytes.len()).ok()
        };
        SecretBytes { bytes, lock }
    }

    /// View the raw bytes. Keep the returned slice's lifetime short.
    pub fn as_slice(&self) -> &[u8] {
        &self.bytes
    }

    /// Whether the buffer was successfully locked into physical RAM.
    pub fn is_locked(&self) -> bool {
        self.lock.is_some()
    }
}

impl Drop for SecretBytes {
    fn drop(&mut self) {
        // Overwrite the secret first, then release the page lock (guard drop).
        self.bytes.zeroize();
        // `lock` (LockGuard) drops after this method returns, unlocking pages.
    }
}
