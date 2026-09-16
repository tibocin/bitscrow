// File: crates/bitscrow-spec/src/digest.rs
// Purpose: Unkeyed blake2b-256 (outlen=32) over bitscrow-jcs-v0 bytes.
// Related: canon.rs, docs/fork_capability_matrix.md (bitscrow-digest)
// Tags: #blake2b #digest #Digest32
//
// Why: same primitive as fork PoW, different message (canonical contract).
// Side effects: none.

use blake2b_simd::Params;

use crate::canon::CanonicalBytes;

/// 32-byte unkeyed BLAKE2b digest of canonical contract bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Digest32(pub [u8; 32]);

/// Hash JCS UTF-8 with unkeyed BLAKE2b, output length 32.
pub fn digest(canonical: &CanonicalBytes) -> Digest32 {
    let hash = Params::new()
        .hash_length(32)
        .to_state()
        .update(&canonical.0)
        .finalize();
    let mut out = [0u8; 32];
    out.copy_from_slice(hash.as_bytes());
    Digest32(out)
}

impl Digest32 {
    /// Hex encoding for logs/tests (not a locator).
    pub fn to_hex(&self) -> String {
        self.0.iter().map(|b| format!("{b:02x}")).collect()
    }
}
