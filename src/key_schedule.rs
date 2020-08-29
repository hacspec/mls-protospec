//! The Key Schedule
//! * HKDF-Extract takes its salt argument from the top and its IKM argument from the left
//! * Derive-Secret takes its Secret argument from the incoming arrow
//!
//!                   init_secret_[n-1]
//!                         |
//!                         V
//!    commit_secret -> KDF.Extract = joiner_secret
//!                         |
//!                         +--> Derive-Secret(., "welcome")
//!                         |    = welcome_secret
//!                         |
//!                         V
//!                   Derive-Secret(., "member")
//!                         |
//!                         V
//!       PSK (or 0) -> KDF.Extract = member_secret
//!                         |
//!                         V
//!                   Derive-Secret(., "epoch")
//!                         |
//!                         V
//! GroupContext_[n] -> KDF.Extract = epoch_secret
//!                         |
//!                         +--> Derive-Secret(., <label>)
//!                         |    = <secret>
//!                         |
//!                         V
//!                   Derive-Secret(., "init")
//!                         |
//!                         V
//!                   init_secret_[n]
use crate::ciphersuites::Ciphersuite;
use evercrypt::hkdf::{hkdf_expand, hkdf_extract};
use evercrypt::hmac;
use hpke;

#[derive(Default)]
struct KeySchedule {
    ciphersuite: Ciphersuite,
}

struct Label {
    group_context_hash: Vec<u8>,
    length: u16,
    label: String, // "mls10 " + Label
    context: Vec<u8>,
}

impl Label {
    fn encode(&self) -> Vec<u8> {
        let mut encoded = group_context_hash.clone();
        encoded.extend(self.length.to_be_bytes());
        encoded.extend(("mls10"+self.label).into_bytes());
        encoded.extend()
    }
}

impl KeySchedule {
    fn hkdf_expand_label(&self, secret: &[u8], label: &Label, length: usize) -> Vec<u8> {
        hkdf_expand(self.kdf, secret, label.encode(), length)
    }
    fn derive_secret(&self, secret: &[u8], label: &str) -> Vec<u8> {
        self.hkdf_expand_label(secret, label, hmac::get_tag_size(self.kdf))
    }
    fn early_secret(&self, init_secret: &[u8], psk: &[u8]) -> Vec<u8> {
        hkdf_extract(self.kdf, init_secret, psk)
    }
}

#[test]
fn test_early_secret() {
    let schedule = KeySchedule::default();
    let _es = schedule.early_secret(&[], &[]);
}
