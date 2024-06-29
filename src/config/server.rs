use std::net::IpAddr;

use axum::http::HeaderValue;
use rising_env_vars::{required_var, var_parsed};

use crate::Env;

use super::{Base, DatabasePools};

pub struct Server {
    pub base: Base,
    pub ip: IpAddr,
    pub port: u16,
    pub max_blocking_threads: Option<usize>,
    pub db: DatabasePools,
    pub allowed_origins: AllowedOrigins,
}

impl Server {
    pub fn from_environment() -> anyhow::Result<Self> {
        let ip = [0, 0, 0, 0].into();
        let port = var_parsed("PORT")?.unwrap_or(9000);
        let allowed_origins = AllowedOrigins::from_default_env()?;
        let base = Base::from_environment()?;
        let max_blocking_threads = var_parsed("SERVER_THREADS")?;

        Ok(Self {
            db: DatabasePools::full_from_environment(&base)?,
            base,
            ip,
            port,
            max_blocking_threads,
            allowed_origins,
        })
    }

    pub fn env(&self) -> Env {
        self.base.env
    }
}

#[derive(Debug, Clone, Default)]
pub struct AllowedOrigins(Vec<String>);

impl AllowedOrigins {
    pub fn from_default_env() -> anyhow::Result<Self> {
        let allowed_origins = required_var("WEB_ALLOWED_ORIGINS")?
            .split(",")
            .map(ToString::to_string)
            .collect();
        Ok(Self(allowed_origins))
    }

    pub fn contains(&self, value: &HeaderValue) -> bool {
        self.0.iter().any(|it| it == value)
    }
}
