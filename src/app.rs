use crate::{
    config,
    db::{connection_url, ConnectionConfig},
};
use axum::extract::{FromRequestParts, State};
use deadpool_diesel::{
    postgres::{Manager as DeadpoolManager, Pool as DeadpoolPool},
    Runtime,
};
use std::{ops::Deref, sync::Arc};

type DeadpoolResult = Result<deadpool_diesel::postgres::Connection, deadpool_diesel::PoolError>;

/// `App` 结构包含应用程序的主要组件,例如数据库连接池和配置
pub struct App {
    /// 连接到主数据库的数据库连接池
    pub primary_database: DeadpoolPool,

    /// 连接到只读副本数据库的数据库连接池
    pub replica_database: Option<DeadpoolPool>,

    /// 服务配置
    pub config: Arc<config::Server>,
}

impl App {
    pub fn new(config: config::Server) -> App {
        let primary_database = {
            use secrecy::ExposeSecret;

            let primary_db_connection_config = ConnectionConfig {
                statement_timeout: config.db.statement_timeout,
                read_only: config.db.primary.read_only_mode,
            };

            let url = connection_url(&config.db, config.db.primary.url.expose_secret());
            let manager = DeadpoolManager::new(url, Runtime::Tokio1);

            DeadpoolPool::builder(manager)
                .runtime(Runtime::Tokio1)
                .max_size(config.db.primary.pool_size)
                .wait_timeout(Some(config.db.connection_timeout))
                .post_create(primary_db_connection_config)
                .build()
                .unwrap()
        };

        let replica_database = if let Some(pool_config) = config.db.replica.as_ref() {
            use secrecy::ExposeSecret;

            let replica_db_connection_config = ConnectionConfig {
                statement_timeout: config.db.statement_timeout,
                read_only: pool_config.read_only_mode,
            };

            let url = connection_url(&config.db, pool_config.url.expose_secret());
            let manager = DeadpoolManager::new(url, Runtime::Tokio1);

            let pool = DeadpoolPool::builder(manager)
                .runtime(Runtime::Tokio1)
                .max_size(pool_config.pool_size)
                .wait_timeout(Some(config.db.connection_timeout))
                .post_create(replica_db_connection_config)
                .build()
                .unwrap();

            Some(pool)
        } else {
            None
        };

        App {
            primary_database,
            replica_database,
            config: Arc::new(config),
        }
    }
}

#[derive(Clone, FromRequestParts)]
#[from_request(via(State))]
pub struct AppState(pub Arc<App>);

// deref so you can still access the inner fields easily
impl Deref for AppState {
    type Target = App;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
