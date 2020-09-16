use crate::{
    ciphersuites::Ciphersuite, credentials::Credential, extensions::*, traits::Encode, encode_util::{encode_slice, LenType}
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
    public_hpke_init_key: Vec<u8>,
    private_hpke_init_key: Vec<u8>,
    credential: Box<Credential>,
    extensions: Vec<Extension>,
    signature: Vec<u8>,
}

impl KeyPackage {
    pub fn new(
        version: ProtocolVersion,
        cipher_suite: Ciphersuite,
        credential: Box<Credential>,
    ) -> Self {
        let hpke_init_key = hpke::Hpke::new(
            hpke::Mode::Base,
            cipher_suite.kem,
            cipher_suite.hpke_kdf,
            cipher_suite.hpke_aead,
        );
        let (sk, pk) = hpke_init_key.key_gen();
        Self {
            version,
            cipher_suite,
            public_hpke_init_key: pk,
            private_hpke_init_key: sk,
            credential,
            extensions: Vec::new(),
            signature: Vec::new(),
        }
    }

    pub(crate) fn add_extension(&mut self, extension: Extension) {
        self.extensions.push(extension);
    }

    pub(crate) fn add_extensions(&mut self, extensions: &[Extension]) {
        // XXX: consume?
        self.extensions.extend_from_slice(extensions);
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.push(self.version as u8);
        out.extend(&self.cipher_suite.encode());
        out.extend(&self.public_hpke_init_key); // This is already encoded.
        out.extend(self.credential.encode());
        out.extend(encode_extensions(&self.extensions));
        out
    }

    pub(crate) fn sign(&mut self, credential: &Credential) {
        let sig = credential.credential.sign(&self.to_bytes());
        self.signature = sig;
    }

    pub(crate) fn verify(&self) -> bool {
        self.credential.credential.verify(&self.to_bytes(), &self.signature)
    }
}

impl Encode for &KeyPackage {
    fn encode(&self) -> Vec<u8> {
        self.to_bytes()
    }
}

// TODO: Add extensions
