// Copyright 2018 Catallaxy

//! Public types for config modules

/// Going to hold all of the various configuration types
/// separately for now, then put them together as a single
/// ServerConfig object afterwards. This is to flatten
/// out the configuration file into logical sections,
/// as they tend to be quite nested in the code
/// Most structs optional, as they may or may not
/// be needed depending on what's being run

use core::types::{SignConfig, VerifyConfig};

#[derive(Clone, Debug)]
pub struct GlobalConfig {
    pub members: Option<ConfigMembers>,
}

/// Keeping an 'inner' structure here, as the top
/// level GlobalConfigContainer options might want to keep
/// internal state that we don't necessarily
/// want serialised or deserialised
#[derive(Clone, Debug)]
pub struct ConfigMembers {
    /// Server config
    pub sign_config: SignConfig,
    /// Logging config
    pub verify_config: VerifyConfig,
}
