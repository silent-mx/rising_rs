pub mod config;
pub mod models;
pub mod schema;
pub mod sentry;
pub mod util;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Env {
    Development,
    Production,
    Test,
}
