use crate::{encode_util::*, traits::Encode};

pub mod ed25519;

type CredentialType = u16;

pub trait CredentialTrait: Encode + std::fmt::Debug {
    fn sign(&self, data: &[u8]) -> Vec<u8>;
    fn verify(&self, data: &[u8], signature: &[u8]) -> bool;
}

#[derive(Debug)]
pub struct Credential {
    credential_type: CredentialType,
    pub(crate) credential: dyn CredentialTrait,
}

impl Encode for Credential {
    fn encode(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend(&self.credential_type.to_be_bytes());
        out.extend(&self.credential.encode());
        out
    }
}

#[derive(Debug)]
pub struct BasicCredential {
    identity: Vec<u8>,
    public_key: Vec<u8>,
    private_key: Vec<u8>,
}

impl CredentialTrait for BasicCredential {
    fn sign(&self, data: &[u8]) -> Vec<u8> {
        unimplemented!()
    }
    fn verify(&self, data: &[u8], signature: &[u8]) -> bool {
        unimplemented!()
    }
}

impl Encode for BasicCredential {
    fn encode(&self) -> Vec<u8> {
        let mut out = Vec::new();
        encode_slice(LenType::L16, &self.identity, &mut out);
        encode_slice(LenType::L16, &self.public_key, &mut out);
        out
    }
}
