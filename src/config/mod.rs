//! User options, held in memory only and never persisted to disk.

/// AES key strength, expressed by raw key size.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum KeySize {
    Aes128,
    Aes256,
}

impl KeySize {
    pub fn bytes(self) -> usize {
        match self {
            KeySize::Aes128 => 16,
            KeySize::Aes256 => 32,
        }
    }

    pub fn hex_len(self) -> usize {
        self.bytes() * 2
    }

    pub fn label(self) -> &'static str {
        match self {
            KeySize::Aes128 => "AES-128  (32 hex chars)",
            KeySize::Aes256 => "AES-256  (64 hex chars)",
        }
    }
}

pub const MAX_KEYS: usize = 100;
pub const MIN_KEYS: usize = 1;

#[derive(Clone)]
pub struct Config {
    pub num_keys: usize,
    pub key_size: KeySize,
    /// DMR programming software varies; uppercase hex is the common default.
    pub uppercase: bool,
    pub start_hidden: bool,

    pub clipboard_wipe_enabled: bool,
    pub clipboard_wipe_secs: u64,

    pub auto_clear_enabled: bool,
    pub auto_clear_secs: u64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            num_keys: 1,
            key_size: KeySize::Aes256,
            uppercase: true,
            start_hidden: false,
            clipboard_wipe_enabled: true,
            clipboard_wipe_secs: 20,
            auto_clear_enabled: true,
            auto_clear_secs: 120,
        }
    }
}
