#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate tracing;
#[macro_use]
extern crate anyhow;

// #[macro_use]
// extern crate serde_json;

pub mod app_state;
pub mod config;
pub mod controllers;
pub mod db;
pub mod middleware;
pub mod models;
pub mod schema;
pub mod utils;

/// 程序的运行环境
/// - `Development`: 开发环境
/// - `Production`: 生产环境
/// - `Test`: 测试环境
///
/// `config.env`的值在*src/bin/server.rs*中设置为
/// - `Production`: 如果环境变量`HOSTNAME`被设置
/// - `Development`: 其他情况
///
/// `config.env`在*src/test/all.rs*中一律为`Test`
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Env {
    Development,
    Production,
    Test,
}
