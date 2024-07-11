use crate::utils::env_vars::required_var;
use argon2::PasswordHasher;
use deadpool_diesel::postgres::{Hook, HookError};
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::time::Duration;
use url::Url;

/// 这会将迁移嵌入到应用程序二进制文件中
/// 迁移路径相对于“CARGO_MANIFEST_DIR”
const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

#[derive(Debug, Clone, Copy)]
pub struct ConnectionConfig {
    pub statement_timeout: Duration,
    pub read_only: bool,
}

impl ConnectionConfig {
    fn apply(&self, conn: &mut PgConnection) -> anyhow::Result<()> {
        diesel::sql_query("SET application_name = 'rising_rs'").execute(conn)?;
        let statement_timeout = self.statement_timeout.as_millis();
        diesel::sql_query(format!("SET statement_timeout = {statement_timeout}")).execute(conn)?;
        if self.read_only {
            diesel::sql_query("SET default_transaction_read_only = 't'").execute(conn)?;
        }

        // 运行数据库迁移
        info!("Migrating the database");
        conn.run_pending_migrations(MIGRATIONS)
            .map_err(|err| anyhow!("Failed to run migrations: {err}"))?;

        // 如果管理员用户不存在,则添加系统管理员
        use crate::models::{NewAdminSysUser, SysUser};
        use crate::schema::sys_user::dsl::*;
        let _ = sys_user
            .filter(is_admin.eq(true))
            .select(SysUser::as_select())
            .first(conn)
            .optional()
            .map(|user: Option<SysUser>| -> anyhow::Result<()> {
                if user.is_none() {
                    info!("Create admin");
                    let password_str = required_var("ADMIN_PASSWORD")?;
                    let argon2 = argon2::Argon2::default();
                    let salt = argon2::password_hash::SaltString::generate(&mut rand_core::OsRng);
                    let hashed_password = argon2
                        .hash_password(password_str.as_bytes(), &salt)
                        .unwrap()
                        .to_string();
                    let new_admin = NewAdminSysUser {
                        username: "admin".to_string(),
                        password: hashed_password,
                        email: None,
                        phone: None,
                        nickname: Some("Admin".to_string()),
                        gender: None,
                        avatar: None,
                        is_admin: true,
                    };
                    diesel::insert_into(sys_user)
                        .values(&new_admin)
                        .execute(conn)?;
                }
                Ok(())
            })?;

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

pub fn connection_url(config: &crate::config::DatabasePools, url: &str) -> String {
    let mut url = Url::parse(url).expect("Invalid database URL");

    if config.enforce_tls {
        maybe_append_url_param(&mut url, "sslmode", "require");
    }

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
