use crate::{
    ciphersuites::Ciphersuite, credential::Credential, extensions::Extension, traits::Encode,
};
use hpke::Hpke;

// Sec. 7 Key Packages
// enum {
//     reserved(0),
//     mls10(1),
//     (255)
// } ProtocolVersion;

// struct {
//     ProtocolVersion version;
//     CipherSuite cipher_suite;
//     HPKEPublicKey hpke_init_key;
//     Credential credential;
//     Extension extensions<8..2^32-1>;
//     opaque signature<0..2^16-1>;
// } KeyPackage;

#[derive(Debug, Copy, Clone)]
pub enum ProtocolVersion {
    Reserved = 0,
    Mls10 = 1,
}

impl Default for ProtocolVersion {
    fn default() -> Self {
        ProtocolVersion::Mls10
    }
}

#[derive(Debug)]
pub struct KeyPackage {
    version: ProtocolVersion,
    cipher_suite: Ciphersuite,
    hpke_init_key: Vec<u8>,
    credential: Box<Credential>,
    extensions: Vec<Extension>,
    signature: Vec<u8>,
}

impl Encode for &KeyPackage {
    fn encode(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.push(self.version as u8);
        out.extend(&self.cipher_suite.encode());
        out.extend(&self.hpke_init_key); // This is already encoded.
        out
    }
}

// TODO: Add extensions
