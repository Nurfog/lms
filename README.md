# Learning Management System (LMS) con Microservicios en Rust

Este proyecto es un Sistema de Gestión de Aprendizaje (LMS) construido utilizando una arquitectura de microservicios con Rust, Axum, SQLx y Docker.

## Arquitectura

El sistema está compuesto por los siguientes microservicios:

- **`identity-service`**: Gestiona todo lo relacionado con la identidad y autenticación de usuarios.
  - Registro de nuevos usuarios.
  - Login y generación de tokens JWT.
  - Puerto: `3000`

- **`course-service`**: Se encargará de la lógica de negocio para cursos, módulos y lecciones.
  - Actualmente, es un esqueleto con la funcionalidad básica para ser expandido.
  - Puerto: `3001`

- **`portal-service`**: Es el punto de entrada web (frontend) para los usuarios. Sirve una aplicación web que consume las APIs de los otros microservicios.
  - Puerto: `8080`

- **`lms-db`**: Una base de datos PostgreSQL centralizada que es compartida por todos los servicios.
  - Puerto: `5432`

## Tech Stack

- **Backend**: Rust
  - **Framework Web**: Axum
  - **Asincronía**: Tokio
  - **Base de Datos**: SQLx con PostgreSQL
- **Base de Datos**: PostgreSQL
- **Contenerización**: Docker & Docker Compose

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