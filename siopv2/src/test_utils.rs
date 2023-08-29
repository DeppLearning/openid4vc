use crate::{StandardClaimsRequests, StandardClaimsValues};
use anyhow::Result;
use async_trait::async_trait;
use derivative::{self, Derivative};
use ed25519_dalek::{Keypair, Signature, Signer};
use lazy_static::lazy_static;
use oid4vc_core::{authentication::sign::ExternalSign, Sign, Subject, Verify};
use rand::rngs::OsRng;
use std::sync::Arc;

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
    pub fn new(did: String, key_id: String) -> Result<Self> {
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

    fn sign(&self, message: &str) -> Result<Vec<u8>, oid4vc_core::error::Error> {
        let signature: Signature = TEST_KEYPAIR.sign(message.as_bytes());
        Ok(signature.to_bytes().to_vec())
    }

    fn external_signer(&self) -> Option<Arc<dyn ExternalSign>> {
        None
    }
}

#[async_trait]
impl Verify for TestSubject {
    async fn public_key(&self, _kid: &str) -> Result<Vec<u8>, oid4vc_core::error::Error> {
        Ok(TEST_KEYPAIR.public.to_bytes().to_vec())
    }
}

impl Subject for TestSubject {
    fn identifier(&self) -> Result<String, oid4vc_core::error::Error> {
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
    async fn public_key(&self, _kid: &str) -> Result<Vec<u8>, oid4vc_core::error::Error> {
        Ok(TEST_KEYPAIR.public.to_bytes().to_vec())
    }
}

pub trait Storage {
    fn fetch_claims(&self, request_claims: &StandardClaimsRequests) -> StandardClaimsValues;
}

#[derive(Default, Debug)]
pub struct MemoryStorage {
    data: StandardClaimsValues,
}

impl MemoryStorage {
    pub fn new(data: StandardClaimsValues) -> Self {
        MemoryStorage { data }
    }
}

impl Storage for MemoryStorage {
    fn fetch_claims(&self, request_claims: &StandardClaimsRequests) -> StandardClaimsValues {
        let mut present = StandardClaimsValues::default();

        macro_rules! present_if {
            ($claim:ident) => {
                if request_claims.$claim.is_some() {
                    present.$claim = self.data.$claim.clone();
                }
            };
        }

        present_if!(name);
        present_if!(family_name);
        present_if!(given_name);
        present_if!(middle_name);
        present_if!(nickname);
        present_if!(preferred_username);
        present_if!(profile);
        present_if!(picture);
        present_if!(website);
        present_if!(gender);
        present_if!(birthdate);
        present_if!(zoneinfo);
        present_if!(locale);
        present_if!(updated_at);
        present_if!(email);
        present_if!(email_verified);
        present_if!(address);
        present_if!(phone_number);
        present_if!(phone_number_verified);

        present
    }
}
