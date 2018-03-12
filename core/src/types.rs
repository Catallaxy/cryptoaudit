// Copyright 2018 Catallaxy

#[derive(Debug, Clone)]
pub struct SignConfig {
    allo: bool,
}
#[derive(Clone, Debug)]
pub struct VerifyConfig {
    bello: bool,
}

impl Default for SignConfig {
    fn default() -> SignConfig {
        SignConfig { allo: false }
    }
}

impl Default for VerifyConfig {
    fn default() -> VerifyConfig {
        VerifyConfig { bello: true }
    }
}
