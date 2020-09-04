use crate::{traits::Encode, encode_util::*};

type CredentialType = u16;

pub trait CredentialTrait: Encode + std::fmt::Debug {}

#[derive(Debug)]
pub struct Credential {
    credential_type: CredentialType,
    credential: dyn CredentialTrait,
}

impl Encode for Credential {
    fn encode(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend(&self.credential_type.to_be_bytes());
        out.extend(&self.credential.encode());
        out
    }
}

// TODO: Add Ed25519 and P256 basic credentials

pub struct BasicCredential {
    identity: Vec<u8>,
    public_key: Vec<u8>,
}

impl Encode for BasicCredential {
    fn encode(&self) -> Vec<u8> {
        let mut out = Vec::new();
        encode_slice(LenType::L16, &self.identity, &mut out);
        encode_slice(LenType::L16, &self.public_key, &mut out);
        out
    }
}
