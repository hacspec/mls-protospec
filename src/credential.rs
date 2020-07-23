type CredentialType = u16;

pub struct Credential {
    credential_type: CredentialType,
    credential: BasicCredential, // TODO: Support other credential types
}

pub struct BasicCredential {
    identity: Vec<u8>,
    public_key: Vec<u8>,
}

// TODO: Add Ed25519 and P256 basic credentials
