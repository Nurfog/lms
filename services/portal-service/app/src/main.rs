use axum::{extract::State, routing::{get, get_service}, Json, Router};
use std::env;
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use serde::{Deserialize, Serialize};
use reqwest;

// --- Estado de la Aplicación ---
#[derive(Clone)]
struct AppState {
    identity_service_url: String,
    course_service_url: String,
}

// --- Estructuras para la API de Endpoints ---
#[derive(Serialize, Deserialize, Debug)]
struct EndpointInfo {
    service: String,
    path: String,
    method: String,
    summary: Option<String>,
}

#[tokio::main]
async fn main() {
    // Inicializar el sistema de logging (tracing)
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
    
    let identity_service_url = env::var("IDENTITY_SERVICE_URL")
        .expect("IDENTITY_SERVICE_URL must be set");
    let course_service_url = env::var("COURSE_SERVICE_URL")
        .expect("COURSE_SERVICE_URL must be set");

    let app_state = AppState {
        identity_service_url,
        course_service_url,
    };

    // Configurar la aplicación para servir archivos desde el directorio "static"
    // `fallback_service` se usa para manejar cualquier petición que no coincida con otras rutas.
    // Aquí, como no hay otras rutas, servirá los archivos estáticos para todas las peticiones.
    let app = Router::new()
        .route("/api/v1/endpoints", get(get_all_endpoints))
        .fallback_service(get_service(ServeDir::new("static")))
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("Portal de bienvenida escuchando en {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// Handler para obtener una lista de todos los endpoints de los microservicios.
async fn get_all_endpoints(State(state): State<AppState>) -> Json<Vec<EndpointInfo>> {
    let client = reqwest::Client::new();
    let mut all_endpoints: Vec<EndpointInfo> = Vec::new();

    // Fetch Identity Service OpenAPI spec
    if let Ok(response) = client.get(&format!("{}/api-docs/openapi.json", state.identity_service_url))
        .send()
        .await
    {
        if let Ok(openapi_json) = response.json::<serde_json::Value>().await {
            if let Some(paths) = openapi_json["paths"].as_object() {
                for (path, methods) in paths {
                    if let Some(methods_obj) = methods.as_object() {
                        for (method, details) in methods_obj {
                            all_endpoints.push(EndpointInfo {
                                service: "Identity".to_string(),
                                path: path.clone(),
                                method: method.to_uppercase(),
                                summary: details["summary"].as_str().map(|s| s.to_string()),
                            });
                        }
                    }
                }
            }
        }
    }

    // Fetch Course Service OpenAPI spec
    if let Ok(response) = client.get(&format!("{}/api-docs/openapi.json", state.course_service_url))
        .send()
        .await
    {
        if let Ok(openapi_json) = response.json::<serde_json::Value>().await {
            // Note: Course Service currently only has health_check, so this will be minimal.
            // We'll expand this when course service gets more endpoints.
            if let Some(paths) = openapi_json["paths"].as_object() {
                for (path, methods) in paths {
                    if let Some(methods_obj) = methods.as_object() {
                        for (method, details) in methods_obj {
                            all_endpoints.push(EndpointInfo {
                                service: "Course".to_string(),
                                path: path.clone(),
                                method: method.to_uppercase(),
                                summary: details["summary"].as_str().map(|s| s.to_string()),
                            });
                        }
                    }
                }
            }
        }
    }

    Json(all_endpoints)
}