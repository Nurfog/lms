use axum::{
    extract::{FromRequestParts, Path, State},
    http::{request::Parts, StatusCode},
    response::IntoResponse,
    routing::{get, post, put},
    Json, Router,
};
use async_trait::async_trait;
use axum_extra::headers::{authorization::Bearer, Authorization};
use axum_extra::TypedHeader;
use jsonwebtoken::{decode, DecodingKey, Validation};
use dotenvy::dotenv;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;
use uuid::Uuid;

// --- Estructuras de Autenticación (copiadas de identity-service) ---

#[derive(Debug, serde::Serialize, serde::Deserialize, ToSchema, Clone, Copy, PartialEq)]
pub enum Role {
    Student,
    Instructor,
    Admin,
}

#[derive(Debug, serde::Deserialize)]
struct Claims {
    sub: Uuid,
    role: Role,
    exp: i64,
}

// --- Estructuras de Datos y Schemas ---

#[derive(serde::Serialize, sqlx::FromRow, ToSchema)]
struct Course {
    id: Uuid,
    instructor_id: Uuid,
    course_name: String,
    course_description: Option<String>,
    course_created_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Payload para crear un nuevo curso.
#[derive(serde::Deserialize, ToSchema)]
struct CreateCourse {
    #[schema(example = "Introducción a Rust")]
    course_name: String,
    #[schema(example = "Un curso para principiantes sobre el lenguaje de programación Rust.")]
    course_description: Option<String>,
}

/// Payload para actualizar un curso.
#[derive(serde::Deserialize, ToSchema)]
struct UpdateCourse {
    #[schema(example = "Introducción a Rust Avanzado")]
    course_name: Option<String>,
    #[schema(example = "Un curso avanzado sobre Rust.")]
    course_description: Option<String>,
}

// --- Estado de la Aplicación ---

#[derive(Clone)]
struct AppState {
    db_pool: PgPool,
    jwt_secret: String,
}

// --- Documentación de la API (OpenAPI) ---

#[derive(OpenApi)]
#[openapi(
    paths(
        health_check,
        create_course
    ),
    components(
        schemas(Course, CreateCourse, Role)
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
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET debe estar configurado");

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("No se pudo conectar a la base de datos");
    
    let app_state = AppState { db_pool, jwt_secret };

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/health", get(health_check))
        .route("/api/v1/courses", post(create_course))
        .route("/api/v1/courses", get(list_courses))
        .route("/api/v1/courses/{id}", get(get_course))
        .route("/api/v1/courses/{id}", put(update_course))
        .with_state(app_state);

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

#[utoipa::path(
    post,
    path = "/api/v1/courses",
    request_body = CreateCourse,
    responses(
        (status = 201, description = "Curso creado exitosamente", body = Course),
        (status = 401, description = "No autorizado (token inválido o ausente)"),
        (status = 403, description = "Prohibido (el usuario no es instructor)"),
        (status = 500, description = "Error interno del servidor")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
async fn create_course(
    State(state): State<AppState>,
    claims: Claims, // El extractor se encargará de la validación del token
    Json(payload): Json<CreateCourse>,
) -> impl IntoResponse {
    // Autorización: Solo los instructores pueden crear cursos.
    if claims.role != Role::Instructor {
        return (StatusCode::FORBIDDEN, "Solo los instructores pueden crear cursos").into_response();
    }

    let new_course_result = sqlx::query_as!(
        Course,
        "INSERT INTO courses (instructor_id, course_name, course_description) VALUES ($1, $2, $3) RETURNING id, instructor_id, course_name, course_description, course_created_at",
        claims.sub, // El ID del instructor viene del token JWT
        payload.course_name,
        payload.course_description
    )
    .fetch_one(&state.db_pool)
    .await;

    match new_course_result {
        Ok(course) => (StatusCode::CREATED, Json(course)).into_response(),
        Err(e) => {
            tracing::error!("Error al crear el curso: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/courses",
    responses(
        (status = 200, description = "Lista de cursos obtenida exitosamente", body = Vec<Course>),
        (status = 500, description = "Error interno del servidor")
    )
)]
async fn list_courses(State(state): State<AppState>) -> impl IntoResponse {
    let courses_result = sqlx::query_as!(
        Course,
        "SELECT id, instructor_id, course_name, course_description, course_created_at FROM courses"
    )
    .fetch_all(&state.db_pool)
    .await;

    match courses_result {
        Ok(courses) => (StatusCode::OK, Json(courses)).into_response(),
        Err(e) => {
            tracing::error!("Error al obtener cursos: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/courses/{id}",
    params(
        ("id" = Uuid, Path, description = "ID del curso")
    ),
    responses(
        (status = 200, description = "Curso obtenido exitosamente", body = Course),
        (status = 404, description = "Curso no encontrado"),
        (status = 500, description = "Error interno del servidor")
    )
)]
async fn get_course(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let course_result = sqlx::query_as!(
        Course,
        "SELECT id, instructor_id, course_name, course_description, course_created_at FROM courses WHERE id = $1",
        id
    )
    .fetch_optional(&state.db_pool)
    .await;

    match course_result {
        Ok(Some(course)) => (StatusCode::OK, Json(course)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            tracing::error!("Error al obtener curso: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[utoipa::path(
    put,
    path = "/api/v1/courses/{id}",
    params(
        ("id" = Uuid, Path, description = "ID del curso")
    ),
    request_body = UpdateCourse,
    responses(
        (status = 200, description = "Curso actualizado exitosamente", body = Course),
        (status = 401, description = "No autorizado"),
        (status = 403, description = "Prohibido (no es el instructor)"),
        (status = 404, description = "Curso no encontrado"),
        (status = 500, description = "Error interno del servidor")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
async fn update_course(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateCourse>,
) -> impl IntoResponse {
    // Verificar que el curso existe y obtener el instructor_id
    let course_check = sqlx::query!(
        "SELECT instructor_id FROM courses WHERE id = $1",
        id
    )
    .fetch_optional(&state.db_pool)
    .await;

    match course_check {
        Ok(Some(course)) => {
            if course.instructor_id != claims.sub {
                return StatusCode::FORBIDDEN.into_response();
            }

            // Actualizar solo los campos proporcionados
            let update_result = sqlx::query_as!(
                Course,
                "UPDATE courses SET 
                    course_name = COALESCE($2, course_name),
                    course_description = COALESCE($3, course_description)
                WHERE id = $1
                RETURNING id, instructor_id, course_name, course_description, course_created_at",
                id,
                payload.course_name,
                payload.course_description
            )
            .fetch_one(&state.db_pool)
            .await;

            match update_result {
                Ok(updated_course) => (StatusCode::OK, Json(updated_course)).into_response(),
                Err(e) => {
                    tracing::error!("Error al actualizar curso: {:?}", e);
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
        }
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            tracing::error!("Error al verificar curso: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

// --- Extractor de Claims JWT ---

impl FromRequestParts<AppState> for Claims {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // Extraer el token de la cabecera `Authorization: Bearer <token>`
        let authorization = parts.headers
            .get("authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer "))
            .ok_or(StatusCode::UNAUTHORIZED)?;

        // Decodificar y validar el token
        let token_data = decode::<Claims>(
            authorization,
            &DecodingKey::from_secret(state.jwt_secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

        Ok(token_data.claims)
    }
}
