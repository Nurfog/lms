# --- Etapa 1: Builder ---
FROM rust:1-alpine AS builder

RUN apk add --no-cache musl-dev curl

WORKDIR /usr/src/app

# --- Cacheo de Dependencias ---
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./services/portal-service/app/Cargo.toml ./services/portal-service/app/Cargo.toml
# Copiamos los otros Cargo.toml para que el workspace sea válido
COPY ./services/identity-service/app/Cargo.toml ./services/identity-service/app/Cargo.toml
COPY ./services/course-service/app/Cargo.toml ./services/course-service/app/Cargo.toml

# Creamos proyectos dummy para que `cargo` pueda descargar y compilar las dependencias.
RUN mkdir -p ./services/portal-service/app/src && \
    echo 'fn main() { println!("Dummy build for caching"); }' > ./services/portal-service/app/src/main.rs && \
    mkdir -p ./services/identity-service/app/src && \
    echo 'fn main() { println!("Dummy build for caching"); }' > ./services/identity-service/app/src/main.rs && \
    mkdir -p ./services/course-service/app/src && \
    echo 'fn main() { println!("Dummy build for caching"); }' > ./services/course-service/app/src/main.rs
RUN cargo build --release --workspace

# Ahora copiamos el código fuente real y los archivos estáticos.
COPY ./services/portal-service/app/src ./services/portal-service/app/src
COPY ./services/portal-service/app/static ./services/portal-service/app/static

# Compilamos el servicio final
RUN cargo build --release --workspace --bin portal-service-app && \
    cp ./target/release/portal-service-app /usr/local/bin/

# --- Etapa 2: Final ---
FROM alpine:latest

COPY --from=builder /usr/src/app/target/release/portal-service-app /usr/local/bin/
COPY --from=builder /usr/src/app/services/portal-service/app/static /static

CMD ["portal-service-app"]
