use axum::{
    routing::get, Router,
};
use dotenvy::dotenv;
use std::env;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

// --- Estructuras de Datos y Schemas ---

// #[derive(serde::Serialize, sqlx::FromRow, ToSchema)]
// struct Course {
//     id: Uuid,
//     course_title: String,
//     course_description: Option<String>,
//     course_created_at: chrono::DateTime<chrono::Utc>,
//     course_updated_at: chrono::DateTime<chrono::Utc>,
// }

// /// (NUEVO) Payload para crear un nuevo curso.
// #[derive(serde::Deserialize, ToSchema)]
// struct CreateCourse {
//     #[schema(example = "Introducción a Rust")]
//     course_title: String,
//     #[schema(example = "Un curso para principiantes sobre el lenguaje de programación Rust.")]
//     course_description: Option<String>,
// }


// --- Documentación de la API (OpenAPI) ---

#[derive(OpenApi)]
#[openapi(
    paths(
        health_check
        // create_course
    ),
    components(
        // schemas(Course, CreateCourse)
    ),
    tags(
        (name = "Course Service", description = "API para gestión de cursos y módulos")
    )
)]
struct ApiDoc;

// --- Lógica Principal y Handlers ---

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/health", get(health_check));
        // .route("/api/v1/courses", post(create_course))

    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    tracing::info!("Servicio de Cursos escuchando en {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Servicio de cursos funcionando", body = String)
    )
)]
async fn health_check() -> &'static str {
    "Course Service: OK"
}
