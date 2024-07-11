mod database_pools;

use crate::utils::env_vars::var_parsed;
use std::net::IpAddr;

pub use database_pools::{DatabasePools, DbPoolConfig};

pub struct Server {
    pub ip: IpAddr,
    pub port: u16,
    pub db: DatabasePools,
}

impl Server {
    pub fn from_environment() -> anyhow::Result<Self> {
        // 加载环境变量
        dotenvy::dotenv().ok();

        let ip = [0, 0, 0, 0].into();
        let port = var_parsed("PORT")?.unwrap_or(9413);

        Ok(Server {
            ip,
            port,
            db: DatabasePools::from_environment()?,
        })
    }
}
