// Copyright 2018 Catallaxy

//! Configuration file management

use types::{GlobalConfig, ConfigMembers};
use core::types::{SignConfig, VerifyConfig};


/// Returns the defaults, as strewn throughout the code
impl Default for ConfigMembers {
    fn default() -> ConfigMembers {
        ConfigMembers {
            sign_config: SignConfig::default(),
            verify_config: VerifyConfig::default(),
        }
    }
}

impl Default for GlobalConfig {
    fn default() -> GlobalConfig {
        GlobalConfig {
            members: Some(ConfigMembers::default()),
        }
    }
}

impl GlobalConfig {
    pub fn new() -> GlobalConfig {
        GlobalConfig::default()
    }
}
