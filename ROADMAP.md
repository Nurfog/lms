# Roadmap del Proyecto LMS

Este documento describe el plan de desarrollo y las futuras funcionalidades para el proyecto LMS.

---

### ✅ Fase 1: Fundación y Autenticación (Completado)

-   [x] Configuración del monorepo con Cargo Workspace.
-   [x] Dockerización de los servicios y la base de datos.
-   [x] Creación de un sistema de migraciones centralizado con `sqlx`.
-   [x] Implementación del **Servicio de Identidad**.
-   [x] Endpoints de registro (`/register`) y login (`/login`).
-   [x] Generación de tokens JWT en el login.
-   [x] Documentación de la API con Swagger UI.
-   [x] Creación de un **Servicio de Portal** como punto de entrada.

---

### 🚧 Fase 2: Servicio de Cursos y Autorización

-   [ ] Implementación del **Servicio de Cursos**.
    -   [ ] Modelo de datos para Cursos y Módulos.
    -   [ ] Endpoints CRUD (Crear, Leer, Actualizar, Borrar) para cursos.
-   [ ] Implementación de middleware de autorización en Axum.
    -   [ ] Proteger rutas que requieran un JWT válido.
    -   [ ] Extraer la información del usuario (ID) desde el token.
-   [ ] Definición de roles de usuario (ej. Estudiante, Instructor, Admin).

---

### 🚀 Fase 3: Funcionalidades Avanzadas y Frontend

-   [ ] Sistema de inscripción de usuarios a cursos.
-   [ ] Gestión de contenido de los módulos (video, texto, etc.).
-   [ ] Creación de una aplicación frontend (ej. con React, Vue, o Svelte) para consumir las APIs.
-   [ ] Pruebas unitarias y de integración para los servicios.
-   [ ] Configuración de un pipeline de CI/CD (ej. con GitHub Actions).