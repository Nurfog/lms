# --- Etapa 1: Builder ---
# Usamos una imagen base de Rust con Debian (Bullseye) para un entorno de compilación más estable.
FROM rust:1-bullseye AS builder

# Establecer el directorio de trabajo.
WORKDIR /usr/src/app

# Copiar todo el workspace.
COPY . .

# Copiar migrations al directorio del crate
RUN cp -r migrations services/course-service/app/migrations

# Compilar el course-service en modo release.
RUN SQLX_OFFLINE=true cargo build --release --bin course-service-app --target x86_64-unknown-linux-gnu

# --- Etapa 2: Runner ---
FROM debian:bookworm-slim AS final
RUN apt-get update && apt-get install -y curl && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-gnu/release/course-service-app .
CMD ["./course-service-app"]