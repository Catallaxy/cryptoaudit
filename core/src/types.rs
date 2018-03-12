// Copyright 2018 Catallaxy

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SignatureAlgorithm{
    SHA256_ECDSA,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Chain {
    Bitcoin,
    Ethereum,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Key {
    key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assets {
    pub chain: String,
    pub keys: Vec<Key>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditedAsset {
    public_key: String,
    signature: String,
    signature_algorithm: SignatureAlgorithm
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditedAssets {
    chain: String,
    assets: Vec<AuditedAsset>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootAuditedAssets {
    message: String,
    audit_asset: Vec<AuditedAsset>,
}

#[derive(Debug, Clone)]
pub struct SignConfig {
    pub file_path: String,
    pub message: String,
}
#[derive(Clone, Debug)]
pub struct VerifyConfig {
    pub file_path: String,
    pub message: String,
}

impl Default for SignConfig {
    fn default() -> SignConfig {
        SignConfig {
            file_path: String::from(""),
            message : String::from(""),
        }
    }
}

impl Default for VerifyConfig {
    fn default() -> VerifyConfig {
        VerifyConfig {
            file_path: String::from(""),
            message : String::from(""),
        }
    }
}
