# Roadmap del Proyecto LMS

Este documento describe las características planificadas y el futuro del proyecto LMS.

## Fase 1: Fundación y Autenticación (Completado)

- [x] Configuración del workspace de Rust con múltiples crates.
- [x] Creación de servicios `identity-service`, `course-service` y `portal-service`.
- [x] Implementación de `docker-compose` para orquestar los servicios.
- [x] Implementación de `healthchecks` para un arranque robusto.
- [x] **`identity-service`**:
  - [x] Endpoint de registro de usuarios (`/register`).
  - [x] Endpoint de login (`/login`) con generación de tokens JWT.
  - [x] Hashing seguro de contraseñas con Argon2.
- [x] Documentación de API con OpenAPI (Swagger UI) para los microservicios.

## Fase 2: Gestión de Cursos y Contenido

- [ ] **`course-service`**:
  - [ ] Implementar el endpoint para crear un nuevo curso (`POST /api/v1/courses`).
  - [ ] Proteger el endpoint de creación de cursos para que solo usuarios con rol `Instructor` o `Admin` puedan usarlo (requiere validación de JWT).
  - [ ] Implementar endpoints para obtener, actualizar y eliminar cursos.
- [ ] **`portal-service`**:
  - [ ] Crear un formulario de login en la UI que consuma el `identity-service`.
  - [ ] Almacenar el JWT de forma segura en el cliente (e.g., `localStorage` o cookies).
  - [ ] Mostrar dinámicamente el estado de login/logout en el header.

## Fase 3: Interacción y UI Avanzada

- [ ] **`portal-service`**:
  - [ ] Crear una página "Mis Cursos" que muestre los cursos en los que un usuario está inscrito.
  - [ ] Desarrollar una UI para que los instructores puedan crear y editar sus cursos.
- [ ] **`course-service`**:
  - [ ] Implementar la lógica para que los estudiantes puedan inscribirse en los cursos.