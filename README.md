# LMS - Sistema de Gestión de Aprendizaje (Monorepo)

Este proyecto es un Sistema de Gestión de Aprendizaje (LMS) construido con una arquitectura de microservicios utilizando Rust, Axum, SQLx, y Docker.

## Tech Stack

-   **Backend**: Rust
-   **Framework Web**: Axum
-   **ORM / Acceso a BD**: SQLx
-   **Base de Datos**: PostgreSQL
-   **Contenerización**: Docker & Docker Compose

## Estructura del Proyecto

El proyecto está organizado como un monorepo de Cargo workspace:

```
lms/
├── .env                # Variables de entorno locales (ignoradas por git)
├── Cargo.lock
├── Cargo.toml          # Workspace principal
├── docker-compose.yml
├── migrations/         # Migraciones de SQLx para la base de datos
├── services/
│   ├── identity-service/
│   │   ├── app/        # Crate del servicio de identidad
│   │   └── Dockerfile
│   ├── course-service/
│   │   ├── app/
│   │   └── Dockerfile
│   └── portal-service/
│       ├── app/        # Crate del portal de bienvenida
│       └── Dockerfile
└── README.md
```

## Prerrequisitos

-   Rust y Cargo
-   Docker y Docker Compose
-   `sqlx-cli` (instalar con `cargo install sqlx-cli`)

## Flujo de Trabajo para Desarrollo

El siguiente flujo es necesario tanto para el desarrollo local como para construir las imágenes de Docker.

1.  **Crear archivo de entorno**:
    Asegúrate de tener un archivo `.env` en la raíz del proyecto con las variables `DATABASE_URL` y `JWT_SECRET`.

2.  **Iniciar la Base de Datos**:
    ```bash
    docker-compose up -d lms-db
    ```

3.  **Ejecutar Migraciones**:
    Aplica el esquema de la base de datos.
    ```bash
    sqlx migrate run
    ```

4.  **Preparar Datos de SQLx (¡Paso Clave!)**:
    Este comando analiza tu código y genera los metadatos necesarios para la compilación offline dentro de Docker. **Debe ejecutarse cada vez que añadas o cambies una consulta SQL**.
    ```bash
    cargo sqlx prepare --workspace
    ```

5.  **Construir y Ejecutar con Docker**:
    ```bash
    docker-compose up --build
    ```

## Endpoints de la API

-   **Identity Service**: `http://localhost:3000`
    -   Documentación Swagger: `http://localhost:3000/swagger-ui`
-   **Course Service**: `http://localhost:3001`
    -   Documentación Swagger: (próximamente)