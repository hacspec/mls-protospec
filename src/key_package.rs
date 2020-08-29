use crate::{ciphersuites::Ciphersuite, credential::Credential, extensions::Extension};

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

pub enum ProtocolVersion {
    Reserved = 0,
    Mls10 = 1,
}

pub struct KeyPackage {
    version: ProtocolVersion,
    cipher_suite: Ciphersuite,
    hpke_init_key: Vec<u8>,
    credential: Credential,
    extensions: Vec<Extension>,
    signature: Vec<u8>,
}

// TODO: Add extensions
