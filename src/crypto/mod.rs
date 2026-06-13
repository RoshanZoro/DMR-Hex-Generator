//! Key generation and hex encoding.
//!
//! Randomness comes from the OS CSPRNG via `getrandom`. Generation fails closed
//! if the entropy source errors, rather than emitting a weak key.

use zeroize::Zeroizing;

use crate::security::SecretBytes;

/// Generate `byte_len` cryptographically random bytes, or an error if the OS
/// entropy source fails.
pub fn generate_key(byte_len: usize) -> Result<SecretBytes, String> {
    let mut buf = vec![0u8; byte_len];
    getrandom::getrandom(&mut buf).map_err(|e| format!("OS RNG failure: {e}"))?;
    Ok(SecretBytes::new(buf))
}

const HEX_UPPER: &[u8; 16] = b"0123456789ABCDEF";
const HEX_LOWER: &[u8; 16] = b"0123456789abcdef";

/// Encode bytes as hex. The returned string zeroizes its buffer on drop.
pub fn to_hex(bytes: &[u8], uppercase: bool) -> Zeroizing<String> {
    let table = if uppercase { HEX_UPPER } else { HEX_LOWER };
    let mut out = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        out.push(table[(b >> 4) as usize] as char);
        out.push(table[(b & 0x0f) as usize] as char);
    }
    Zeroizing::new(out)
}
