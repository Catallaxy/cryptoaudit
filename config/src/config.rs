// Copyright 2018 Catallaxy
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Configuration file management

use types::{ConfigMembers, GlobalConfig};
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
