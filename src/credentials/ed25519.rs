use crate::credentials::*;
use evercrypt::prelude::ed25519;

impl BasicCredential {
    pub fn ed25519() -> Self {
        let sk = ed25519::key_gen();
        let pk = ed25519::sk2pk(&sk);
        Self {
            identity: pk.to_vec(),
            public_key: pk.to_vec(),
            private_key: sk.to_vec(),
        }
    }
}
