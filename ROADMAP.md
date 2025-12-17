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
  - [x] Implementar el endpoint para crear un nuevo curso (`POST /api/v1/courses`). (Completado)
  - [x] Proteger el endpoint de creación de cursos para que solo usuarios con rol `Instructor` o `Admin` puedan usarlo (requiere validación de JWT). (Completado)
  - [x] Implementar endpoints para obtener cursos (`GET /api/v1/courses`). (Completado)
  - [x] Implementar endpoint para obtener un curso específico (`GET /api/v1/courses/{id}`). (Completado)
  - [ ] Implementar endpoint para actualizar un curso (`PUT /api/v1/courses/{id}`), solo para instructores del curso.
  - [ ] Implementar endpoint para eliminar un curso (`DELETE /api/v1/courses/{id}`), solo para instructores o admins.
  - [ ] Agregar paginación y filtros a los endpoints de cursos.
- [ ] **`portal-service`**:
  - [ ] Crear un formulario de login en la UI que consuma el `identity-service`.
  - [ ] Almacenar el JWT de forma segura en el cliente (e.g., `localStorage` o cookies).
  - [ ] Mostrar dinámicamente el estado de login/logout en el header.
  - [ ] Crear una página para listar cursos disponibles.
  - [ ] Integrar la creación de cursos desde el portal para instructores.

## Fase 3: Interacción y UI Avanzada

- [ ] **`portal-service`** (LMS - Learning Management System):
  - [ ] Separar la UI en vistas diferenciadas:
    - **Vista de Estudiantes**: Página principal con cursos disponibles, "Mis Cursos" inscritos, progreso de aprendizaje, y acceso a contenido de cursos.
    - **Vista de Instructores**: Dashboard para gestionar cursos propios, ver estadísticas de estudiantes, y acceso rápido a Studio.
  - [ ] Crear un formulario de login en la UI que consuma el `identity-service`.
  - [ ] Almacenar el JWT de forma segura en el cliente (e.g., `localStorage` o cookies).
  - [ ] Mostrar dinámicamente el estado de login/logout en el header, con navegación basada en rol (estudiante/instructor).
  - [ ] Implementar navegación responsiva y moderna, similar a Open edX LMS.
- [ ] **`studio-service`** (Nuevo servicio para creación y administración de cursos):
  - [ ] Crear un servicio separado o módulo en portal-service para Studio.
  - [ ] UI para instructores: Crear/editar cursos, agregar módulos, lecciones, quizzes, y multimedia.
  - [ ] Vista de estructura de cursos: Árbol jerárquico de módulos y lecciones.
  - [ ] Herramientas de autoría: Editor de texto enriquecido, subida de archivos, configuración de fechas y permisos.
  - [ ] Integración con course-service para guardar cambios en la base de datos.
- [ ] **`course-service`**:
  - [ ] Implementar la lógica para que los estudiantes puedan inscribirse en los cursos (enrolamiento).
  - [ ] Agregar gestión de módulos y lecciones dentro de cursos (estructura jerárquica).
  - [ ] Implementar progreso de aprendizaje por usuario (tracking de lecciones completadas).
  - [ ] Soporte para diferentes tipos de contenido: texto, video, quizzes, cuestionarios y pruebas.
  - [ ] Integración LTI para BigBlueButton (videoconferencias en vivo para lecciones).
  - [ ] Agregar descripciones detalladas a cursos, módulos y lecciones.
- [ ] **`file-service`** (Nuevo servicio para manejo de archivos):
  - [ ] Implementar subida y gestión de archivos estáticos (imágenes, videos, PDFs) para cursos.
  - [ ] Soporte para archivos subidos por usuarios (instructores para materiales de cursos).
  - [ ] Almacenamiento seguro, con validación de tipos y tamaños.
  - [ ] Integración con course-service para asociar archivos a cursos/módulos.
  - [ ] Agregar gestión de módulos y lecciones dentro de cursos.
  - [ ] Implementar progreso de aprendizaje por usuario.

## Fase 4: Testing, Seguridad y Despliegue

- [ ] Agregar pruebas unitarias e integración para todos los servicios.
- [ ] Implementar CI/CD con GitHub Actions para builds automáticos y tests.
- [ ] Mejorar la seguridad: rate limiting, validación de inputs, manejo de errores.
- [ ] Configurar despliegue en contenedores (Kubernetes o similar).
- [ ] Agregar logging centralizado y monitoreo (e.g., Prometheus, Grafana).
- [ ] Documentar APIs completas y guías de contribución.