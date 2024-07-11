use std::time::Duration;

use secrecy::SecretString;

use crate::utils::env_vars::{required_var, var_parsed};

#[derive(Debug)]
pub struct DbPoolConfig {
    pub url: SecretString,
    pub read_only_mode: bool,
    pub pool_size: usize,
    pub min_idle: Option<u32>,
}

pub struct DatabasePools {
    pub primary: DbPoolConfig,
    pub tcp_timeout_ms: u64,
    pub connection_timeout: Duration,
    pub statement_timeout: Duration,
    /// 用于异步操作的线程数，例如连接创建
    pub helper_threads: usize,
    /// 是否强制要求所有数据库连接都使用 TLS 加密
    pub enforce_tls: bool,
}

impl DatabasePools {
    const DEFAULT_POOL_SIZE: usize = 3;

    /// 从环境变量加载设置
    pub fn from_environment() -> anyhow::Result<Self> {
        let database_url = required_var("DATABASE_URL")?.into();
        let pool_size = var_parsed("DATABASE_POOL_SIZE")?.unwrap_or(Self::DEFAULT_POOL_SIZE);
        let min_idle = var_parsed("DATABASE_POOL_MIN_IDLE")?;
        let tcp_timeout_ms = var_parsed("DATABASE_TCP_TIMEOUT_MS")?.unwrap_or(15 * 1000);
        let connection_timeout =
            Duration::from_secs(var_parsed("DATABASE_CONNECTION_TIMEOUT")?.unwrap_or(30));
        let statement_timeout = connection_timeout;
        let helper_threads = var_parsed("DATABASE_HELPER_THREADS")?.unwrap_or(3);

        // 如果在release下运行，设置enforce_tls为true
        let enforce_tls = std::env::var("RUST_ENV")
            .map(|s| s == "release")
            .unwrap_or(false);

        Ok(DatabasePools {
            primary: DbPoolConfig {
                url: database_url,
                read_only_mode: false,
                pool_size,
                min_idle,
            },
            tcp_timeout_ms,
            connection_timeout,
            statement_timeout,
            helper_threads,
            enforce_tls,
        })
    }
}
