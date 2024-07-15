#[macro_use]
extern crate tracing;

use std::net::SocketAddr;
use tokio::signal::unix::{signal, SignalKind};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const CORE_THREADS: usize = 4;

fn main() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rising_rs=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 从环境变量初始化app配置
    let config = rising_rs::config::Server::from_environment()?;

    // 构建 Axum Router
    let axum_router = rising_rs::controllers::build_axum_router(config.clone());

    // 创建 Tokio Runtime并设定线程数
    let mut builder = tokio::runtime::Builder::new_multi_thread();
    builder.enable_all();
    builder.worker_threads(CORE_THREADS);
    let rt = builder.build().unwrap();
    let make_service = axum_router.into_make_service_with_connect_info::<SocketAddr>();

    // 阻止主线程，直到服务器关闭
    rt.block_on(async {
        // 使用tokio创建TCP监听器
        let listener = tokio::net::TcpListener::bind((config.ip, config.port)).await?;
        let addr = listener.local_addr()?;

        info!("Listening at http://{addr}");

        // Run the server with gracefull shutdown
        axum::serve(listener, make_service)
            .with_graceful_shutdown(shutdown_signal())
            .await
    })?;

    info!("Server has gracefully shutdown!");
    Ok(())
}

async fn shutdown_signal() {
    let interrupt = async {
        signal(SignalKind::interrupt())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    let terminate = async {
        signal(SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = interrupt => {},
        _ = terminate => {},
    }
}
