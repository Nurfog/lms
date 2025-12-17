use axum::{
    routing::get, Router,
};
use dotenvy::dotenv;
use sqlx::{postgres::PgPoolOptions, PgPool};
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
    eprintln!("Starting course service");
    dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL debe estar configurada");

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("No se pudo conectar a la base de datos");

    // Ejecutar migraciones
    sqlx::migrate!("./migrations").run(&db_pool).await.expect("Failed to run migrations");

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/health", get(health_check))
        .with_state(db_pool);
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
