use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use dotenvy::dotenv;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;
use uuid::Uuid;

// (NUEVO) Enum para los roles de usuario, debe coincidir con el tipo SQL
#[derive(Debug, Serialize, Deserialize, sqlx::Type, ToSchema, Clone, Copy)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum Role {
    Student,
    Instructor,
    Admin,
}

// --- Estructuras de Datos y Schemas para OpenAPI ---

/// Representa un usuario en la base de datos.
#[derive(Serialize, sqlx::FromRow, ToSchema)]
struct User {
    id: Uuid,
    first_name: String,
    last_name: String,
    username: String,
    email: String,
    role: Role, // (NUEVO)
    created_at: chrono::DateTime<chrono::Utc>,
    // El hash de la contraseña nunca debe ser expuesto en una respuesta de API.
    // Lo mantenemos aquí porque `query_as!` lo necesita para mapear el resultado.
    #[serde(skip_serializing)]
    password_hash: String,
}

/// Datos para crear un nuevo usuario (payload de registro).
#[derive(Deserialize, ToSchema)]
struct CreateUser {
    #[schema(example = "Juan")]
    first_name: String,
    #[schema(example = "Allende")]
    last_name: String,
    #[schema(example = "nurfog")]
    username: String,
    #[schema(example = "test@example.com")]
    email: String,
    #[schema(example = "SecurePassword123")]
    password: String,
}

/// Respuesta pública de un usuario (sin datos sensibles).
#[derive(Serialize, ToSchema)]
struct UserResponse {
    id: Uuid,
    first_name: String,
    last_name: String,
    username: String,
    email: String,
    role: Role, // (NUEVO)
}

/// (NUEVO) Payload para el endpoint de login.
#[derive(Deserialize, ToSchema)]
struct LoginPayload {
    // El login puede ser con email o username, aquí usamos email.
    #[schema(example = "test@example.com")]
    email: String,
    #[schema(example = "SecurePassword123")]
    password: String,
}

/// (NUEVO) Respuesta del endpoint de login, contiene el token.
#[derive(Serialize, ToSchema)]
struct TokenResponse {
    token: String,
}

/// (NUEVO) Estructura de los claims (contenido) del JWT.
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: Uuid, // Subject (el ID del usuario)
    role: Role, // (NUEVO) Rol del usuario
    exp: i64,  // Expiration time
}

// --- Estado de la Aplicación ---

#[derive(Clone)]
struct AppState {
    db_pool: PgPool,
    jwt_secret: String, // (NUEVO) Secreto para firmar los JWT
}

// --- Documentación de la API (OpenAPI) ---

#[derive(OpenApi)]
#[openapi(
    paths(
        health_check,
        register,
        login
    ),
    components(
        schemas(User, CreateUser, UserResponse, LoginPayload, TokenResponse)
    ),
    tags(
        (name = "Identity Service", description = "API para gestión de usuarios y autenticación")
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

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL debe estar configurada");
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET debe estar configurado"); // (NUEVO)

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("No se pudo conectar a la base de datos");

    // Ejecutar las migraciones centralizadas
    sqlx::migrate!("../../../migrations")
        .run(&db_pool)
        .await
        .expect("No se pudieron ejecutar las migraciones de la base de datos");

    let app_state = AppState { db_pool, jwt_secret }; // (NUEVO)

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/health", get(health_check))
        .route("/api/v1/auth/register", post(register))
        .route("/api/v1/auth/login", post(login)) // (NUEVO)
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Escuchando en {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Servicio de identidad funcionando", body = String)
    )
)]
async fn health_check() -> &'static str {
    "Identity Service: OK"
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/register",
    request_body = CreateUser,
    responses(
        (status = 201, description = "Usuario creado exitosamente", body = UserResponse),
        (status = 409, description = "El email ya está en uso"),
        (status = 500, description = "Error interno del servidor")
    )
)]
async fn register(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Response {
    let password_hash = match hash(payload.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let new_user = sqlx::query_as!(
        User,
        "INSERT INTO users (first_name, last_name, username, email, password_hash) VALUES ($1, $2, $3, $4, $5) RETURNING id, first_name, last_name, username, email, password_hash, role as \"role: _\", created_at as \"created_at!\"",
        payload.first_name,
        payload.last_name,
        payload.username,
        payload.email.to_lowercase(),
        password_hash
    )
    .fetch_one(&state.db_pool)
    .await;

    match new_user {
        Ok(user) => {
            let user_response = UserResponse {
                id: user.id,
                email: user.email,
                first_name: user.first_name,
                last_name: user.last_name,
                username: user.username,
                role: user.role,
            };
            (StatusCode::CREATED, Json(user_response)).into_response()
        }
        Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
            StatusCode::CONFLICT.into_response()
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    request_body = LoginPayload,
    responses(
        (status = 200, description = "Login exitoso, devuelve token JWT", body = TokenResponse),
        (status = 401, description = "Credenciales inválidas"),
        (status = 500, description = "Error interno del servidor")
    )
)]
/// Handler para el login de usuarios
async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginPayload>,
) -> Response {
    // 1. Buscar al usuario por email
    let user = match sqlx::query_as!(
        User,
        "SELECT id, first_name, last_name, username, email, password_hash, role as \"role: _\", created_at as \"created_at!\" FROM users WHERE email = $1",
        payload.email.to_lowercase(),
    )
    .fetch_optional(&state.db_pool)
    .await
    {
        Ok(Some(user)) => user,
        Ok(None) => return StatusCode::UNAUTHORIZED.into_response(), // Usuario no encontrado
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    // 2. Verificar la contraseña
    let password_valid = match verify(&payload.password, &user.password_hash) {
        Ok(valid) => valid,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    if !password_valid {
        return StatusCode::UNAUTHORIZED.into_response(); // Contraseña incorrecta
    }

    // 3. Crear el token JWT
    let claims = Claims {
        sub: user.id,
        role: user.role,
        exp: (Utc::now() + Duration::hours(24)).timestamp(), // Expira en 24 horas
    };

    let token = match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_ref()),
    ) {
        Ok(t) => t,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    // 4. Devolver el token
    (StatusCode::OK, Json(TokenResponse { token })).into_response()
}