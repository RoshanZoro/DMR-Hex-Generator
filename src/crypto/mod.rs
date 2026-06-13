//! Key generation and hex encoding.
//!
//! Randomness comes straight from the operating system CSPRNG via `getrandom`
//! (BCryptGenRandom on Windows, getrandom(2)/getentropy on Linux/macOS). This
//! is the same class of source as Python's `secrets` module: suitable for
//! generating cryptographic keys.

use zeroize::Zeroizing;

use crate::security::SecretBytes;

/// Generate one secret of `byte_len` cryptographically random bytes.
///
/// Returns an error only if the OS entropy source itself fails, in which case
/// no key material is produced (fail closed rather than emit weak keys).
pub fn generate_key(byte_len: usize) -> Result<SecretBytes, String> {
    let mut buf = vec![0u8; byte_len];
    getrandom::getrandom(&mut buf).map_err(|e| format!("OS RNG failure: {e}"))?;
    Ok(SecretBytes::new(buf))
}

const HEX_UPPER: &[u8; 16] = b"0123456789ABCDEF";
const HEX_LOWER: &[u8; 16] = b"0123456789abcdef";

/// Encode bytes as a hex string. The returned string zeroizes its buffer when
/// dropped, so secret hex never lingers in freed heap memory.
pub fn to_hex(bytes: &[u8], uppercase: bool) -> Zeroizing<String> {
    let table = if uppercase { HEX_UPPER } else { HEX_LOWER };
    let mut out = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        out.push(table[(b >> 4) as usize] as char);
        out.push(table[(b & 0x0f) as usize] as char);
    }
    Zeroizing::new(out)
}
