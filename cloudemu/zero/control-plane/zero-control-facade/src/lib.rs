use axum::{
    extract::{Path, State},
    http::{StatusCode, HeaderMap},
    response::{IntoResponse, Response},
    routing::any,
    Router,
};
use std::sync::Arc;
use zero_control_core::ZeroProvider;
use zero_control_spi::{ZeroRequest, ZeroService};
use zero_data_core::ZeroEngine;

pub struct ServerState {
    pub provider: Arc<ZeroProvider>,
}

pub async fn start_server(port: u16, native: bool, mock: bool) -> anyhow::Result<()> {
    // Pre-flight checks
    check_wsl_preflight();

    // 1. Initialize Engine
    let engine = if mock {
        ZeroEngine::mock_local().map_err(|e| anyhow::anyhow!(e))?
    } else if native {
        ZeroEngine::native().map_err(|e| anyhow::anyhow!(e))?
    } else {
        ZeroEngine::auto().map_err(|e| anyhow::anyhow!(e))?
    };
    
    let provider = Arc::new(ZeroProvider::new(Arc::new(engine)));
    let state = Arc::new(ServerState { provider });

    // 2. Setup CORS
    let cors = tower_http::cors::CorsLayer::permissive();

    // 3. Setup Routes
    let app = Router::new()
        .route("/*path", any(handler))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    tracing::info!("ZeroCloud API listening on http://0.0.0.0:{}", port);
    axum::serve(listener, app).await?;

    Ok(())
}

async fn handler(
    State(state): State<Arc<ServerState>>,
    method: axum::http::Method,
    Path(path): Path<String>,
    headers: HeaderMap,
    body: axum::body::Bytes,
) -> impl IntoResponse {
    let mut zero_headers = std::collections::HashMap::new();
    for (name, value) in headers.iter() {
        zero_headers.insert(name.to_string(), value.to_str().unwrap_or("").to_string());
    }

    let req = ZeroRequest {
        method: method.to_string(),
        path: format!("/{}", path),
        headers: zero_headers,
        body: body.to_vec(),
    };

    match state.provider.handle_request(req).await {
        Ok(resp) => {
            let mut axum_resp = Response::builder()
                .status(StatusCode::from_u16(resp.status).unwrap_or(StatusCode::OK));
            
            for (name, value) in resp.headers {
                axum_resp = axum_resp.header(name, value);
            }

            axum_resp.body(axum::body::Body::from(resp.body)).unwrap()
        }
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}

fn check_wsl_preflight() {
    #[cfg(target_os = "linux")]
    {
        let is_wsl = std::fs::read_to_string("/proc/version")
            .map(|v| v.to_lowercase().contains("microsoft") || v.to_lowercase().contains("wsl"))
            .unwrap_or(false);

        if is_wsl {
             let kvm_exists = std::path::Path::new("/dev/kvm").exists();
             if !kvm_exists {
                 tracing::warn!("WSL 2 detected but /dev/kvm is missing!");
                 tracing::warn!("To enable high-performance native orchestration, enable nested virtualization in .wslconfig.");
             }
        }
    }
}
