#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid subject syntax type: {0}")]
    InvalidSubjectSyntaxType(String),
    #[error("Invalid DID method: {0}")]
    InvalidDidMethodError(String),
    #[error("Invalid DID URL: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("Invalid DID URL: {0}")]
    JsonWebTokenError(#[from] jsonwebtoken::errors::Error),
    #[error("Invalid DID URL: {0}")]
    MissingKeyIdentifierError(String),
    #[error("Invalid DID URL: {0}")]
    MissingValidatorError(String),
    #[error("Invalid DID URL: {0}")]
    DidUrlError(#[from] did_url::Error),
}
