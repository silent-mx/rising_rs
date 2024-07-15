use deadpool_diesel::{
    postgres::{Manager as DeadpoolManager, Pool as DeadPgPool},
    Runtime,
};

use crate::db::{connection_url, ConnectionConfig};

type DeadpoolResult = Result<deadpool_diesel::postgres::Connection, deadpool_diesel::PoolError>;

#[derive(Clone)]
pub struct AppState {
    pub pg_pool: DeadPgPool,
    pub config: crate::config::Server,
}

impl AppState {
    pub fn new(config: crate::config::Server) -> Self {
        let pg_pool = {
            use secrecy::ExposeSecret;

            let pg_connection_config = ConnectionConfig {
                statement_timeout: config.db_pool.statement_timeout,
            };
            let pg_url = connection_url(&config.db_pool, config.db_pool.url.expose_secret());
            let pool_manager = DeadpoolManager::new(pg_url, Runtime::Tokio1);

            DeadPgPool::builder(pool_manager)
                .runtime(Runtime::Tokio1)
                .max_size(config.db_pool.pool_size)
                .wait_timeout(Some(config.db_pool.connection_timeout))
                .post_create(pg_connection_config)
                .build()
                .unwrap()
        };

        AppState { pg_pool, config }
    }

    #[instrument(skip_all)]
    pub async fn pg_pool(&self) -> DeadpoolResult {
        self.pg_pool.get().await
    }
}
