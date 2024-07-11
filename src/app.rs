use std::{ops::Deref, sync::Arc};

use axum::extract::{FromRequestParts, State};
use deadpool_diesel::{
    postgres::{Manager as DeadpoolManager, Pool as DeadpoolPool},
    Runtime,
};

use crate::{
    config::Server,
    db::{connection_url, ConnectionConfig},
};

type DeadpoolResult = Result<deadpool_diesel::postgres::Connection, deadpool_diesel::PoolError>;

/// `App`结构包含应用程序的主要组,例如数据库连接池和配置
pub struct App {
    /// 连接到主数据库的数据库连接池
    pub primary_database: DeadpoolPool,
    /// App 配置
    pub config: Arc<Server>,
}

impl App {
    pub fn new(config: Server) -> App {
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

        App {
            primary_database,
            config: Arc::new(config),
        }
    }

    /// Obtain a read/write database connection from the async primary pool
    #[instrument(skip_all)]
    pub async fn db_write(&self) -> DeadpoolResult {
        self.primary_database.get().await
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

// impl FromRef<AppState> for cookie::Key {
//     fn from_ref(app: &AppState) -> Self {
//         app.session_key().clone()
//     }
// }
