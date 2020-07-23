///! The Key Schedule
///! * HKDF-Extract takes its salt argument from the top and its IKM argument from the left
///! * Derive-Secret takes its Secret argument from the incoming arrow
///!
///!                init_secret_[n-1] (or 0)
///!                      |
///!                      V
///!     PSK (or 0) -> HKDF-Extract = early_secret
///!                      |
///!                Derive-Secret(., "derived", "")
///!                      |
///!                      V
///! commit_secret -> HKDF-Extract = epoch_secret
///!                      |
///!                      +--> HKDF-Expand(., "mls 1.0 welcome", Hash.length)
///!                      |    = welcome_secret
///!                      |
///!                      +--> Derive-Secret(., "sender data", GroupContext_[n])
///!                      |    = sender_data_secret
///!                      |
///!                      +--> Derive-Secret(., "handshake", GroupContext_[n])
///!                      |    = handshake_secret
///!                      |
///!                      +--> Derive-Secret(., "app", GroupContext_[n])
///!                      |    = application_secret
///!                      |
///!                      +--> Derive-Secret(., "exporter", GroupContext_[n])
///!                      |    = exporter_secret
///!                      |
///!                      +--> Derive-Secret(., "confirm", GroupContext_[n])
///!                      |    = confirmation_key
///!                      |
///!                      V
///!                Derive-Secret(., "init", GroupContext_[n])
///!                      |
///!                      V
///!                init_secret_[n]
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
