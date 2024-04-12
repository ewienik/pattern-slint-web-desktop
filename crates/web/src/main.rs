use {
    axum::Router,
    std::net::SocketAddr,
    tower_http::{
        services::{ServeDir, ServeFile},
        trace::TraceLayer,
    },
    tracing_subscriber::prelude::*,
    tracing_subscriber::{fmt, EnvFilter},
};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("info"))
                .unwrap(),
        )
        .with(fmt::layer().with_target(false))
        .init();
    let addr = SocketAddr::from(([0, 0, 0, 0], 6080));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        Router::new()
            .nest_service("/", ServeFile::new("index.html"))
            .nest_service("/pkg", ServeDir::new("pkg"))
            .layer(TraceLayer::new_for_http()),
    )
    .await
    .unwrap();
}
