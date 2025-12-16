# --- Etapa 1: Builder ---
# Usamos una imagen de Rust con Alpine Linux, que es ligera.
FROM rust:1-alpine AS builder

# Instalamos las dependencias de construcción necesarias.
# 'musl-dev' es para la compilación estática y 'curl' para descargar paquetes.
RUN apk add --no-cache musl-dev curl

WORKDIR /usr/src/app

# --- Cacheo de Dependencias ---
# Copiamos los archivos Cargo del workspace y del servicio para cachear las dependencias.
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./services/identity-service/app/Cargo.toml ./services/identity-service/app/Cargo.toml
COPY ./services/course-service/app/Cargo.toml ./services/course-service/app/Cargo.toml
COPY ./services/portal-service/app/Cargo.toml ./services/portal-service/app/Cargo.toml
# (NUEVO) Copiamos los datos de sqlx y las migraciones para la compilación offline.
COPY ./.sqlx ./.sqlx
COPY ./migrations ./migrations

# Creamos un proyecto dummy para que `cargo` pueda descargar y compilar las dependencias.
RUN mkdir -p ./services/identity-service/app/src && \
    echo 'fn main() { println!("Dummy build for caching"); }' > ./services/identity-service/app/src/main.rs && \
    mkdir -p ./services/course-service/app/src && \
    echo 'fn main() { println!("Dummy build for caching"); }' > ./services/course-service/app/src/main.rs && \
    mkdir -p ./services/portal-service/app/src && \
    echo 'fn main() { println!("Dummy build for caching"); }' > ./services/portal-service/app/src/main.rs
RUN SQLX_OFFLINE=true cargo build --release --workspace

# Ahora copiamos el código fuente real de la aplicación.
COPY ./services/identity-service/app/src ./services/identity-service/app/src

# Compilamos el servicio específico, aprovechando las dependencias cacheadas.
RUN SQLX_OFFLINE=true cargo build --release --workspace --bin identity-service-app && \
    cp ./target/release/identity-service-app /usr/local/bin/

# --- Etapa 2: Final ---
# Usamos una imagen base mínima para la imagen final.
FROM alpine:latest
COPY --from=builder /usr/local/bin/identity-service-app /usr/local/bin/
CMD ["identity-service-app"]