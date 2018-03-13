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
    pub addresses: Vec<String>,
    pub public_key: String,
    pub signature: String,
    pub signature_algorithm: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditedAssets {
    pub chain: String,
    pub assets: Vec<AuditedAsset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootAuditedAssets {
    pub message: String,
    pub message_digest: String,
    pub audited_assets: Vec<AuditedAssets>,
}

impl Default for RootAuditedAssets {
    fn default() -> RootAuditedAssets {
        RootAuditedAssets {
            message: String::from(""),
            message_digest: String::from(""),
            audited_assets: Vec::new(),
        }
    }
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
            message: String::from(""),
        }
    }
}

impl Default for VerifyConfig {
    fn default() -> VerifyConfig {
        VerifyConfig {
            file_path: String::from(""),
            message: String::from(""),
        }
    }
}
