use anyhow::Result;
use axum::{
    extract::{Path as AxumPath, State},
    http::StatusCode,
    routing::get,
    Router,
};
use std::{net::SocketAddr, ops::Deref, path::PathBuf, sync::Arc};
use tokio::{fs, net::TcpListener};
use tower_http::services::ServeDir;
use tracing::info;

struct ServeHttpState {
    path: PathBuf,
}

#[derive(Clone)]
struct ServeHttp(Arc<ServeHttpState>);
impl ServeHttp {
    fn new(path: PathBuf) -> Self {
        Self(Arc::new(ServeHttpState { path }))
    }
}

impl Deref for ServeHttp {
    type Target = ServeHttpState;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub async fn process_http_serve(path: &PathBuf, port: u16) -> Result<()> {
    info!("Serve: path: {path:?}, port: {port}");
    let state = ServeHttp::new(path.clone());
    let serve_dir = ServeDir::new(path);
    let router = Router::new()
        .route("/", get(index_handler))
        .nest_service("/tower", serve_dir)
        .route("/*file", get(file_handler))
        .with_state(state);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn index_handler() -> &'static str {
    "Hello World!"
}

async fn file_handler(
    State(state): State<ServeHttp>,
    AxumPath(file): AxumPath<String>,
) -> (StatusCode, String) {
    let f = state.path.join(file);
    let f = f.as_path();
    if f.exists() {
        // TODO: dir list
        match fs::read_to_string(f).await {
            Ok(content) => (StatusCode::OK, content),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        }
    } else {
        (StatusCode::NOT_FOUND, "Not Found".to_string())
    }
}
