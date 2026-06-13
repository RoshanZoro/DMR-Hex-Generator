//! Storage for raw key material: a heap buffer that is memory-locked against
//! swap (best effort) and zeroized on drop. This is the only place raw key
//! bytes are held.

use zeroize::Zeroize;

/// A heap-allocated secret that is zeroized on drop and locked into RAM.
pub struct SecretBytes {
    bytes: Vec<u8>,
    /// Page lock for `bytes`; dropping it unlocks the region.
    lock: Option<region::LockGuard>,
}

impl SecretBytes {
    /// Take ownership of `bytes`, attempting to lock its pages into RAM. The
    /// lock is best effort; check [`SecretBytes::is_locked`] for the result.
    pub fn new(bytes: Vec<u8>) -> Self {
        let lock = if bytes.is_empty() {
            None
        } else {
            // `bytes` is never grown, so this pointer stays valid until the
            // guard is dropped in `Drop`.
            region::lock(bytes.as_ptr(), bytes.len()).ok()
        };
        SecretBytes { bytes, lock }
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.bytes
    }

    /// Whether the buffer is locked into physical RAM.
    pub fn is_locked(&self) -> bool {
        self.lock.is_some()
    }
}

impl Drop for SecretBytes {
    fn drop(&mut self) {
        // Wipe the secret; the lock guard then drops, unlocking the pages.
        self.bytes.zeroize();
    }
}
