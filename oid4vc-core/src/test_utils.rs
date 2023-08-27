use std::sync::Arc;

use crate::{authentication::sign::ExternalSign, Sign, Subject, Verify};
use async_trait::async_trait;
use derivative::{self, Derivative};
use ed25519_dalek::{Keypair, Signature, Signer};
use lazy_static::lazy_static;
use rand::rngs::OsRng;

// Keypair for mocking purposes.
lazy_static! {
    pub static ref TEST_KEYPAIR: Keypair = Keypair::generate(&mut OsRng);
}

#[derive(Derivative)]
#[derivative(Default)]
pub struct TestSubject {
    #[derivative(Default(value = "identity_did::DIDUrl::parse(\"did:test:123\").unwrap()"))]
    pub did: identity_did::DIDUrl,
    pub key_id: String,
}

impl TestSubject {
    pub fn new(did: String, key_id: String) -> Result<Self, crate::error::Error> {
        Ok(TestSubject {
            did: identity_did::DIDUrl::parse(did)?,
            key_id,
        })
    }
}

impl Sign for TestSubject {
    fn key_id(&self) -> Option<String> {
        Some(self.key_id.clone())
    }

    fn sign(&self, message: &str) -> Result<Vec<u8>, crate::error::Error> {
        let signature: Signature = TEST_KEYPAIR.sign(message.as_bytes());
        Ok(signature.to_bytes().to_vec())
    }

    fn external_signer(&self) -> Option<Arc<dyn ExternalSign>> {
        None
    }
}

#[async_trait]
impl Verify for TestSubject {
    async fn public_key(&self, _kid: &str) -> Result<Vec<u8>, crate::error::Error> {
        Ok(TEST_KEYPAIR.public.to_bytes().to_vec())
    }
}

impl Subject for TestSubject {
    fn identifier(&self) -> Result<String, crate::error::Error> {
        Ok(self.did.to_string())
    }
}

pub struct MockVerifier;

impl MockVerifier {
    pub fn new() -> Self {
        MockVerifier {}
    }
}

#[async_trait]
impl Verify for MockVerifier {
    async fn public_key(&self, _kid: &str) -> Result<Vec<u8>, crate::error::Error> {
        Ok(TEST_KEYPAIR.public.to_bytes().to_vec())
    }
}
