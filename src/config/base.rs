//! 基础配置选项
//!
//! - `CONTAINER_ID`: 表明程序在Docker容器中运行

use crate::Env;
use rising_env_vars::var;

pub struct Base {
    pub env: Env,
}

impl Base {
    pub fn from_environment() -> anyhow::Result<Self> {
        let env = match var("CONTAINER_ID")? {
            Some(_) => Env::Production,
            _ => Env::Development,
        };

        Ok(Self { env })
    }
}
