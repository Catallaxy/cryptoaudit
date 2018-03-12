// Copyright 2018 Catallaxy

#[derive(Debug, Clone)]
pub struct SignConfig {
    pub file: String,
    pub message: String,
}
#[derive(Clone, Debug)]
pub struct VerifyConfig {
    pub file: String,
    pub message: String,
}

impl Default for SignConfig {
    fn default() -> SignConfig {
        SignConfig {
            file: String::from(""),
            message : String::from(""),
        }
    }
}

impl Default for VerifyConfig {
    fn default() -> VerifyConfig {
        VerifyConfig {
            file: String::from(""),
            message : String::from(""),
        }
    }
}
