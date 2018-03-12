// Copyright 2018 Catallaxy

/// The files part aka in
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Key {
    pub key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assets {
    pub chain: String,
    pub keys: Vec<Key>,
}

/// The auditing types aka out
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditedAsset {
    pub public_key: String,
    pub signature: String,
    pub signature_algorithm: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditedAssets {
    pub chain: String,
    pub assets: Vec<AuditedAsset>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootAuditedAssets {
    pub message: String,
    pub audited_assets: Vec<AuditedAssets>,
}

// Configuration types
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
