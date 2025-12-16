# --- Etapa 1: Builder ---
# Usamos la imagen oficial de Rust para compilar nuestra aplicación.
FROM rust:1-alpine as builder

# Instalamos dependencias de build.
RUN apk add --no-cache musl-dev curl

WORKDIR /usr/src/app

# --- Cacheo de Dependencias ---
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./services/course-service/app/Cargo.toml ./services/course-service/app/Cargo.toml
COPY ./services/identity-service/app/Cargo.toml ./services/identity-service/app/Cargo.toml
COPY ./services/portal-service/app/Cargo.toml ./services/portal-service/app/Cargo.toml
# (NUEVO) Copiamos los datos de sqlx y las migraciones para la compilación offline.
COPY ./.sqlx ./.sqlx
COPY ./migrations ./migrations

RUN mkdir -p ./services/course-service/app/src && \
    echo 'fn main() { println!("Dummy build for caching"); }' > ./services/course-service/app/src/main.rs && \
    mkdir -p ./services/identity-service/app/src && \
    echo 'fn main() { println!("Dummy build for caching"); }' > ./services/identity-service/app/src/main.rs && \
    mkdir -p ./services/portal-service/app/src && \
    echo 'fn main() { println!("Dummy build for caching"); }' > ./services/portal-service/app/src/main.rs
RUN SQLX_OFFLINE=true cargo build --release --workspace

# Copiamos el código fuente real.
COPY ./services/course-service/app/src ./services/course-service/app/src

# Compilamos el servicio específico.
RUN SQLX_OFFLINE=true cargo build --release --workspace --bin course-service-app && \
    cp ./target/release/course-service-app /usr/local/bin/

# --- Etapa 2: Final ---
# Usamos una imagen base mínima para reducir el tamaño final.
FROM alpine:latest
COPY --from=builder /usr/local/bin/course-service-app /usr/local/bin/
CMD ["course-service-app"]