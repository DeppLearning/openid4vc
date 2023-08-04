pub mod iso_mdl;
pub mod w3c_verifiable_credentials;

use self::{
    iso_mdl::mso_mdoc::MsoMdoc,
    sealed::FormatExtension,
    w3c_verifiable_credentials::{jwt_vc_json::JwtVcJson, jwt_vc_json_ld::JwtVcJsonLd, ldp_vc::LdpVc},
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[macro_export]
macro_rules! credential_format {
    ($format:literal, $name:ty, {$($field_name:ident: $field_type:ty),*}) => {
        paste::paste! {
            #[derive(Debug, Clone, Eq, PartialEq, Default)]
            pub struct $name;
            impl $crate::credential_format_profiles::Format for $name {
                type Parameters = [< $name Parameters >];
                type Credential = serde_json::Value;
            }

            #[serde_with::skip_serializing_none]
            #[derive(Debug, serde::Serialize, serde::Deserialize, Eq, PartialEq, Clone)]
            pub struct [< $name Parameters >] {
                $(pub $field_name: $field_type),*
            }

            #[serde_with::skip_serializing_none]
            #[derive(Debug, serde::Serialize, serde::Deserialize, Eq, PartialEq, Clone)]
            pub struct [< $name Credential >] {
                $(pub $field_name: $field_type),*
            }

            #[allow(unused_parens)]
            impl From<($($field_type),*)> for [< $name Parameters >] {
                fn from(($($field_name),*): ($($field_type),*)) -> Self {
                    Self {
                        $($field_name),*
                    }
                }
            }

            $crate::serialize_unit_struct!($format, $name);
        }
    };
}

pub trait Format: std::fmt::Debug + Serialize + Eq + PartialEq + Default + Send + Sync {
    type Parameters: std::fmt::Debug + Serialize + DeserializeOwned + Eq + PartialEq + Clone + Send + Sync;
    type Credential: std::fmt::Debug + Serialize + DeserializeOwned + Eq + PartialEq + Clone + Send + Sync;
}

pub trait CredentialFormatCollection: Serialize + Send + Sync + Clone {}

#[derive(Debug, Serialize, Clone, Eq, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum CredentialFormats<C = WithParameters>
where
    C: FormatExtension + DeserializeOwned,
{
    JwtVcJson(C::Container<JwtVcJson>),
    JwtVcJsonLd(C::Container<JwtVcJsonLd>),
    LdpVc(C::Container<LdpVc>),
    MsoMdoc(C::Container<MsoMdoc>),
    Other(serde_json::Value),
}
impl<C> CredentialFormatCollection for CredentialFormats<C> where C: FormatExtension {}

mod sealed {
    use super::Format;
    use serde::{de::DeserializeOwned, Deserialize, Serialize};

    pub trait FormatExtension: Serialize + Clone + Sync + Send + DeserializeOwned {
        type Container<F: Format + Clone + Sync + Send + for<'de> Deserialize<'de>>: Serialize
            + Clone
            + Sync
            + Send
            + for<'de> Deserialize<'de>
            + DeserializeOwned;
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct WithParameters;
impl FormatExtension for WithParameters {
    type Container<F: Format + Clone + Sync + Send + for<'de> Deserialize<'de>> = Parameters<F>;
}
#[derive(Debug, Serialize, Clone, Eq, PartialEq, Deserialize)]
pub struct Parameters<F>
where
    F: Format,
{
    pub format: F,
    #[serde(flatten)]
    pub parameters: F::Parameters,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct WithCredential;
impl FormatExtension for WithCredential {
    type Container<F: Format + Clone + Sync + Send + for<'de> Deserialize<'de>> = Credential<F>;
}
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Credential<F>
where
    F: Format,
{
    pub format: F,
    pub credential: F::Credential,
}
