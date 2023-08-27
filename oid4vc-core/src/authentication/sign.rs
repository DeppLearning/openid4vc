use std::sync::Arc;

pub trait Sign: Send + Sync {
    // TODO: add this?
    // fn jwt_alg_name() -> &'static str;
    fn key_id(&self) -> Option<String>;
    fn sign(&self, message: &str) -> Result<Vec<u8>, crate::error::Error>;
    fn external_signer(&self) -> Option<Arc<dyn ExternalSign>>;
}

pub trait ExternalSign: Send + Sync {
    fn sign(&self, message: &str) -> Result<Vec<u8>, crate::error::Error>;
}
