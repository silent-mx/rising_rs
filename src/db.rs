use deadpool_diesel::postgres::{Hook, HookError};
use diesel::prelude::*;
use std::time::Duration;
use url::Url;

use crate::config;

#[derive(Debug, Clone, Copy)]
pub struct ConnectionConfig {
    pub statement_timeout: Duration,
    pub read_only: bool,
}

impl ConnectionConfig {
    fn apply(&self, conn: &mut PgConnection) -> QueryResult<()> {
        diesel::sql_query("SET application_name = 'rising_rs'").execute(conn)?;
        let statement_timeout = self.statement_timeout.as_millis();
        diesel::sql_query(format!("SET statement_timeout = {statement_timeout}")).execute(conn)?;

        if self.read_only {
            diesel::sql_query("SET default_transaction_read_only = 't'").execute(conn)?;
        }

        Ok(())
    }
}

impl From<ConnectionConfig> for Hook {
    fn from(config: ConnectionConfig) -> Self {
        Hook::async_fn(move |conn, _| {
            Box::pin(async move {
                conn.interact(move |conn| config.apply(conn))
                    .await
                    .map_err(|err| HookError::message(err.to_string()))?
                    .map_err(|err| HookError::message(err.to_string()))
            })
        })
    }
}

pub fn connection_url(config: &config::DatabasePools, url: &str) -> String {
    let mut url = Url::parse(url).expect("Invalid database url");

    if config.enforce_tls {
        maybe_append_url_param(&mut url, "sslmode", "require");
    }

    // Configure the time it takes for diesel to return an error when there is full packet loss
    // between the application and the database.
    maybe_append_url_param(
        &mut url,
        "tcp_user_timeout",
        &config.tcp_timeout_ms.to_string(),
    );

    url.into()
}

fn maybe_append_url_param(url: &mut Url, key: &str, value: &str) {
    if !url.query_pairs().any(|(k, _)| k == key) {
        url.query_pairs_mut().append_pair(key, value);
    }
}
