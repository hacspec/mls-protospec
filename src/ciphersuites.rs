use evercrypt::{aead, digest, hmac, signature};
use hpke;
use crate::traits::Encode;

#[derive(Debug, PartialEq)]
pub struct Ciphersuite {
    pub(crate) name: Name,
    pub(crate) hash: digest::Mode,
    pub(crate) kem: hpke::kem::Mode,
    pub(crate) kdf: hmac::Mode, // Not in spec. Only HKDF is specified here. This can't be used in HPKE, but only standalone
    pub(crate) hpke_kdf: hpke::kdf::Mode,
    pub(crate) hpke_aead: hpke::aead::Mode, // Not in spec. Should really be the same as aead.
    pub(crate) aead: aead::Mode,
    pub(crate) signature: signature::Mode,
}

impl Encode for Ciphersuite {
    fn encode(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&(self.name as u16).to_be_bytes());
        out
    }
}

/// The default ciphersuite is the only mandatory one.
impl Default for Ciphersuite {
    fn default() -> Self {
        Self::new(Name::MLS10_128_DHKEMX25519_AES128GCM_SHA256_Ed25519)
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Name {
    MLS10_128_DHKEMX25519_AES128GCM_SHA256_Ed25519 = 0x0001,
    MLS10_128_DHKEMP256_AES128GCM_SHA256_P256 = 0x0002,
    MLS10_128_DHKEMX25519_CHACHA20POLY1305_SHA256_Ed25519 = 0x0003,
    MLS10_256_DHKEMX448_AES256GCM_SHA512_Ed448 = 0x0004,
    MLS10_256_DHKEMP521_AES256GCM_SHA512_P521 = 0x0005,
    MLS10_256_DHKEMX448_CHACHA20POLY1305_SHA512_Ed448 = 0x0006,
}

fn get_hash_from_suite(name: &Name) -> digest::Mode {
    match name {
        Name::MLS10_128_DHKEMX25519_AES128GCM_SHA256_Ed25519 => digest::Mode::Sha256,
        Name::MLS10_128_DHKEMP256_AES128GCM_SHA256_P256 => digest::Mode::Sha256,
        Name::MLS10_128_DHKEMX25519_CHACHA20POLY1305_SHA256_Ed25519 => digest::Mode::Sha256,
        Name::MLS10_256_DHKEMX448_AES256GCM_SHA512_Ed448 => digest::Mode::Sha512,
        Name::MLS10_256_DHKEMP521_AES256GCM_SHA512_P521 => digest::Mode::Sha512,
        Name::MLS10_256_DHKEMX448_CHACHA20POLY1305_SHA512_Ed448 => digest::Mode::Sha512,
    }
}

fn get_aead_from_suite(name: &Name) -> aead::Mode {
    match name {
        Name::MLS10_128_DHKEMX25519_AES128GCM_SHA256_Ed25519 => aead::Mode::Aes128Gcm,
        Name::MLS10_128_DHKEMP256_AES128GCM_SHA256_P256 => aead::Mode::Aes128Gcm,
        Name::MLS10_128_DHKEMX25519_CHACHA20POLY1305_SHA256_Ed25519 => aead::Mode::Chacha20Poly1305,
        Name::MLS10_256_DHKEMX448_AES256GCM_SHA512_Ed448 => aead::Mode::Aes256Gcm,
        Name::MLS10_256_DHKEMP521_AES256GCM_SHA512_P521 => aead::Mode::Aes256Gcm,
        Name::MLS10_256_DHKEMX448_CHACHA20POLY1305_SHA512_Ed448 => aead::Mode::Chacha20Poly1305,
    }
}

fn get_signature_from_suite(name: &Name) -> signature::Mode {
    match name {
        Name::MLS10_128_DHKEMX25519_AES128GCM_SHA256_Ed25519 => signature::Mode::Ed25519,
        Name::MLS10_128_DHKEMP256_AES128GCM_SHA256_P256 => signature::Mode::P256,
        Name::MLS10_128_DHKEMX25519_CHACHA20POLY1305_SHA256_Ed25519 => signature::Mode::Ed25519,
        _ => panic!(
            "Signature scheme for ciphersuite {:?} is not implemented yet.",
            name
        ),
    }
}

fn get_kem_from_suite(name: &Name) -> hpke::kem::Mode {
    match name {
        Name::MLS10_128_DHKEMX25519_AES128GCM_SHA256_Ed25519 => hpke::kem::Mode::DhKem25519,
        Name::MLS10_128_DHKEMP256_AES128GCM_SHA256_P256 => hpke::kem::Mode::DhKemP256,
        Name::MLS10_128_DHKEMX25519_CHACHA20POLY1305_SHA256_Ed25519 => hpke::kem::Mode::DhKem25519,
        _ => panic!("KEM for ciphersuite {:?} is not implemented yet.", name),
    }
}

fn get_kdf_from_suite(name: &Name) -> hmac::Mode {
    match name {
        Name::MLS10_128_DHKEMX25519_AES128GCM_SHA256_Ed25519
        | Name::MLS10_128_DHKEMP256_AES128GCM_SHA256_P256
        | Name::MLS10_128_DHKEMX25519_CHACHA20POLY1305_SHA256_Ed25519 => hmac::Mode::Sha256,
        Name::MLS10_256_DHKEMX448_AES256GCM_SHA512_Ed448
        | Name::MLS10_256_DHKEMP521_AES256GCM_SHA512_P521
        | Name::MLS10_256_DHKEMX448_CHACHA20POLY1305_SHA512_Ed448 => hmac::Mode::Sha512,
    }
}

fn get_hpke_aead_from_suite(name: &Name) -> hpke::aead::Mode {
    match name {
        Name::MLS10_128_DHKEMX25519_AES128GCM_SHA256_Ed25519 => hpke::aead::Mode::AesGcm128,
        Name::MLS10_128_DHKEMP256_AES128GCM_SHA256_P256 => hpke::aead::Mode::AesGcm128,
        Name::MLS10_128_DHKEMX25519_CHACHA20POLY1305_SHA256_Ed25519 => {
            hpke::aead::Mode::ChaCha20Poly1305
        }
        Name::MLS10_256_DHKEMX448_AES256GCM_SHA512_Ed448 => hpke::aead::Mode::AesGcm256,
        Name::MLS10_256_DHKEMP521_AES256GCM_SHA512_P521 => hpke::aead::Mode::AesGcm256,
        Name::MLS10_256_DHKEMX448_CHACHA20POLY1305_SHA512_Ed448 => {
            hpke::aead::Mode::ChaCha20Poly1305
        }
    }
}

fn get_hpke_kdf_from_suite(name: &Name) -> hpke::kdf::Mode {
    match name {
        Name::MLS10_128_DHKEMX25519_AES128GCM_SHA256_Ed25519 => hpke::kdf::Mode::HkdfSha256,
        Name::MLS10_128_DHKEMP256_AES128GCM_SHA256_P256 => hpke::kdf::Mode::HkdfSha256,
        Name::MLS10_128_DHKEMX25519_CHACHA20POLY1305_SHA256_Ed25519 => {
            hpke::kdf::Mode::HkdfSha256
        }
        Name::MLS10_256_DHKEMX448_AES256GCM_SHA512_Ed448 => hpke::kdf::Mode::HkdfSha512,
        Name::MLS10_256_DHKEMP521_AES256GCM_SHA512_P521 => hpke::kdf::Mode::HkdfSha512,
        Name::MLS10_256_DHKEMX448_CHACHA20POLY1305_SHA512_Ed448 => {
            hpke::kdf::Mode::HkdfSha512
        }
    }
}

impl Ciphersuite {
    pub fn new(name: Name) -> Self {
        Self {
            name: name,
            hash: get_hash_from_suite(&name),
            kem: get_kem_from_suite(&name),
            kdf: get_kdf_from_suite(&name),
            hpke_kdf: get_hpke_kdf_from_suite(&name),
            hpke_aead: get_hpke_aead_from_suite(&name),
            aead: get_aead_from_suite(&name),
            signature: get_signature_from_suite(&name),
        }
    }
    pub fn get_name(&self) -> &Name {
        &self.name
    }
}

impl From<&Name> for u16 {
    fn from(s: &Name) -> u16 {
        *s as u16
    }
}

impl From<u16> for Ciphersuite {
    fn from(v: u16) -> Self {
        let name = match v {
            0x0001 => Name::MLS10_128_DHKEMX25519_AES128GCM_SHA256_Ed25519,
            0x0002 => Name::MLS10_128_DHKEMP256_AES128GCM_SHA256_P256,
            0x0003 => Name::MLS10_128_DHKEMX25519_CHACHA20POLY1305_SHA256_Ed25519,
            0x0004 => Name::MLS10_256_DHKEMX448_AES256GCM_SHA512_Ed448,
            0x0005 => Name::MLS10_256_DHKEMP521_AES256GCM_SHA512_P521,
            0x0006 => Name::MLS10_256_DHKEMX448_CHACHA20POLY1305_SHA512_Ed448,
            _ => panic!("Not implemented."),
        };
        Self::new(name)
    }
}

impl From<Name> for Ciphersuite {
    fn from(v: Name) -> Self {
        Self::new(v)
    }
}

#[test]
fn test_ciphersuite() {
    assert_eq!(Ciphersuite::from(1), Ciphersuite::default());
}
