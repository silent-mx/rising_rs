//! 用于设置数据库池的配置
//!
//! - `DATABASE_URL`: 要使用的postgres数据库的URL
//! - `DB_POOL_SIZE`: 数据库连接池最大连接数,默认10
//! - `DB_TCP_TIMEOUT_MS`: TCP连接超时(以毫秒为单位),默认15秒
//! - `DB_TIMEOUT`: 数据库响应超时时长(以秒为单位),默认30秒

use crate::utils::env_vars::{required_var, var_parsed};
use secrecy::SecretString;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct DatabasePool {
    /// postgres数据库连接字符串
    pub url: SecretString,
    /// 数据库连接池最大连接数,默认10
    pub pool_size: usize,
    /// 数据库TCP连接超时(毫秒)
    pub tcp_timeout_ms: u64,
    /// 等待连接从连接池变为可用的时长
    pub connection_timeout: Duration,
    /// 在查询响应之前，等待取消查询的时长
    pub statement_timeout: Duration,
}

impl DatabasePool {
    const DEFAULT_POOL_SIZE: usize = 10;

    /// 从环境变量加载配置
    pub fn from_environment() -> anyhow::Result<Self> {
        let database_url = required_var("DATABASE_URL")?.into();
        let pool_size = var_parsed("DB_POOL_SIZE")?.unwrap_or(Self::DEFAULT_POOL_SIZE);
        let tcp_timeout_ms = var_parsed("DB_TCP_TIMEOUT_MS")?.unwrap_or(15 * 1000);
        let connection_timeout = Duration::from_secs(var_parsed("DB_TIMEOUT")?.unwrap_or(30));
        let statement_timeout = connection_timeout;
        Ok(DatabasePool {
            url: database_url,
            pool_size,
            tcp_timeout_ms,
            connection_timeout,
            statement_timeout,
        })
    }
}
