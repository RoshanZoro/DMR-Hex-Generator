//! User-configurable options. Held only in RAM; never persisted to disk so
//! that nothing about a session survives the process.

/// AES key strength, expressed by raw key size.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum KeySize {
    Aes128,
    Aes256,
}

impl KeySize {
    /// Raw key length in bytes.
    pub fn bytes(self) -> usize {
        match self {
            KeySize::Aes128 => 16,
            KeySize::Aes256 => 32,
        }
    }

    /// Length of the hex representation in characters.
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

/// Hard limits to keep the UI responsive and memory bounded.
pub const MAX_KEYS: usize = 100;
pub const MIN_KEYS: usize = 1;

#[derive(Clone)]
pub struct Config {
    /// How many keys to generate per click.
    pub num_keys: usize,
    /// AES key strength.
    pub key_size: KeySize,
    /// Render hex in uppercase (DMR CPS programs vary; uppercase is common).
    pub uppercase: bool,
    /// Start with keys masked on screen (reveal individually).
    pub start_hidden: bool,

    /// Wipe the clipboard automatically after a copy.
    pub clipboard_wipe_enabled: bool,
    /// Seconds before the clipboard is wiped.
    pub clipboard_wipe_secs: u64,

    /// Drop all generated keys from RAM automatically after a delay.
    pub auto_clear_enabled: bool,
    /// Seconds before keys are auto-cleared.
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
