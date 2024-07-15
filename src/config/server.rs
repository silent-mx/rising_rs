use std::net::IpAddr;

use crate::{utils::env_vars::var_parsed, Env};

use super::{base::Base, database_pool::DatabasePool};

#[derive(Debug, Clone)]
pub struct Server {
    pub base: Base,
    pub ip: IpAddr,
    pub port: u16,
    pub db_pool: DatabasePool,
}

impl Server {
    /// 返回程序配置
    ///
    /// ## Panics
    /// 如果配置无效,服务启动失败
    pub fn from_environment() -> anyhow::Result<Self> {
        // 在本地通过cargo run运行程序, 可把环境变量写入`.env`文件，通过[dotenvy]加载
        if cfg!(debug_assertions) {
            dotenvy::dotenv().ok();
        }

        let base = Base::from_environment()?;
        let ip = [0, 0, 0, 0].into();
        let port = var_parsed("PORT")?.unwrap_or(9413);
        let db_pool = DatabasePool::from_environment()?;

        Ok(Self {
            base,
            ip,
            port,
            db_pool,
        })
    }

    pub fn env(&self) -> Env {
        self.base.env
    }
}
