# Learning Management System (LMS) con Microservicios en Rust

Este proyecto es un Sistema de Gestión de Aprendizaje (LMS) construido utilizando una arquitectura de microservicios con Rust, Axum, SQLx y Docker.

## Arquitectura

El sistema está compuesto por los siguientes microservicios:

- **`identity-service`**: Gestiona todo lo relacionado con la identidad y autenticación de usuarios.
  - Registro de nuevos usuarios.
  - Login y generación de tokens JWT.
  - Puerto: `3000`

- **`course-service`**: Se encargará de la lógica de negocio para cursos, módulos, lecciones, enrolamiento de estudiantes, cuestionarios y pruebas.
  - Incluye integración LTI para BigBlueButton.
  - Puerto: `3001`

- **`file-service`**: Gestiona la subida y administración de archivos estáticos y materiales de cursos.
  - Puerto: `3002` (propuesto)

- **`studio-service`**: Herramienta de autoría para instructores, similar a Open edX Studio, para crear y administrar la estructura de cursos.
  - Puerto: `8081` (propuesto)

- **`portal-service`**: Es el punto de entrada web (frontend) para los usuarios. Sirve una aplicación web que consume las APIs de los otros microservicios.
  - Incluye vistas diferenciadas para estudiantes e instructores (similar a Open edX LMS).
  - Puerto: `8080`

- **`studio-service`**: Herramienta de autoría para instructores, similar a Open edX Studio, para crear y administrar la estructura de cursos.
  - Puerto: `8081` (propuesto)

- **`lms-db`**: Una base de datos PostgreSQL centralizada que es compartida por todos los servicios.
  - Puerto: `5432`

## Tech Stack

- **Backend**: Rust
  - **Framework Web**: Axum
  - **Asincronía**: Tokio
  - **Base de Datos**: SQLx con PostgreSQL
  - **Autenticación**: JWT
  - **Integraciones**: LTI para BigBlueButton, manejo de archivos
- **Base de Datos**: PostgreSQL
- **Contenerización**: Docker & Docker Compose
- **Frontend**: HTML/CSS/JS servido por Axum (para vistas LMS y Studio)

## Cómo Empezar

### Prerrequisitos

- Docker
- Docker Compose

### Ejecución

1. Clona el repositorio.
2. Desde la raíz del proyecto, ejecuta el siguiente comando para construir y levantar todos los servicios:
   ```bash
   docker compose up --build
   ```

### Endpoints Disponibles

- **Portal Web**: http://localhost:8080
- **API Docs (Identity Service)**: http://localhost:3000/swagger-ui
- **API Docs (Course Service)**: http://localhost:3001/swagger-ui