use anyhow::{anyhow, Context};
use std::{error::Error, str::FromStr};

/// 从当前进程中读取键值为`key`的环境变量
///
/// 和[std::env::var]相比有如下不同
/// - 使用[dotenvy]可以从`.env`文件加载环境变量
/// - 当环境变量没有被设置，返回`Ok(None)`(而不是`Err`)
#[track_caller]
pub fn var(key: &str) -> anyhow::Result<Option<String>> {
    match dotenvy::var(key) {
        Ok(content) => Ok(Some(content)),
        Err(dotenvy::Error::EnvVar(std::env::VarError::NotPresent)) => Ok(None),
        Err(error) => Err(error.into()),
    }
}

/// 从当前进程中读取键值为`key`的环境变量，如果没有设置，返回`Err`
///
/// 和[std::env::var]相比，[required_var]使用[dotenvy]从`.env`
/// 文件中加载环境变量
#[track_caller]
pub fn required_var(key: &str) -> anyhow::Result<String> {
    required(var(key), key)
}

/// 读取当前进程的环境变量，并解析
///
/// 和[std::env::var]相比有如下不同
/// - 使用[dotenvy]可以从`.env`文件加载环境变量
/// - 当环境变量没有被设置，返回`Ok(None)`(而不是`Err`)
#[track_caller]
pub fn var_parsed<T>(key: &str) -> anyhow::Result<Option<T>>
where
    T: FromStr,
    T::Err: Error + Send + Sync + 'static,
{
    match var(key) {
        Ok(Some(content)) => {
            Ok(Some(content.parse().with_context(|| {
                format!("Failed to parse {key} environment variable")
            })?))
        }
        Ok(None) => Ok(None),
        Err(error) => Err(error),
    }
}
/// 从当前进程中读取键值为`key`的环境变量并解析，如果没有设置，返回`Err`
///
/// 和[std::env::var]相比，[required_var]使用[dotenvy]从`.env`
/// 文件中加载环境变量
#[track_caller]
pub fn required_var_parsed<T>(key: &str) -> anyhow::Result<T>
where
    T: FromStr,
    T::Err: Error + Send + Sync + 'static,
{
    required(var_parsed(key), key)
}

/// 读取键值为`key`的环境变量，并按逗号分隔解析为`Vec<String>`,
/// 如果没设置则返回空数组
#[track_caller]
pub fn list(key: &str) -> anyhow::Result<Vec<String>> {
    let values = match var(key)? {
        None => vec![],
        Some(s) if s.is_empty() => vec![],
        Some(s) => s.split(',').map(str::trim).map(String::from).collect(),
    };

    Ok(values)
}

/// 从环境变量中读取并按逗号分隔解析成list
/// 如果环境变量为空，则返回空数组
/// 每个单独的值都是使用 [FromStr] 解析
#[track_caller]
pub fn list_parsed<T, E, F, C>(key: &str, f: F) -> anyhow::Result<Vec<T>>
where
    F: Fn(&str) -> C,
    C: Context<T, C>,
{
    let values = match var(key)? {
        None => vec![],
        Some(s) if s.is_empty() => vec![],
        Some(s) => s
            .split(',')
            .map(str::trim)
            .map(|s| {
                f(s).with_context(|| {
                    format!("Failed to parse value \"{s}\" of {key} environment variable")
                })
            })
            .collect::<Result<_, _>>()?,
    };

    Ok(values)
}

fn required<T>(res: anyhow::Result<Option<T>>, key: &str) -> anyhow::Result<T> {
    match res {
        Ok(opt) => opt.ok_or_else(|| anyhow!("Failed to find required {key} environment variable")),
        Err(error) => Err(error),
    }
}
