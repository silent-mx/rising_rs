//! 基础配置
//!
//! - `HOSTNAME`: 程序运行在docker容器中

use crate::{utils::env_vars::var, Env};

#[derive(Debug, Clone)]
pub struct Base {
    pub env: Env,
}

impl Base {
    pub fn from_environment() -> anyhow::Result<Self> {
        let env = match var("HOSTNAME")? {
            Some(_) => Env::Production,
            _ => Env::Development,
        };

        Ok(Self { env })
    }
}
