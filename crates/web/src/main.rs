use {
    axum::{
        extract::{Path, State},
        routing, Router,
    },
    backend::Backend,
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
            .route("/counter/:counter", routing::get(process_counter))
            .nest_service("/pkg", ServeDir::new("pkg"))
            .layer(TraceLayer::new_for_http())
            .with_state(Backend::new()),
    )
    .await
    .unwrap();
}

async fn process_counter(State(be): State<Backend>, Path(counter): Path<i32>) -> String {
    dbg!(i32::from(be.process_counter(counter.into()).await).to_string())
}
