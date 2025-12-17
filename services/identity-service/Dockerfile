# --- Etapa 1: Builder ---
# Usamos una imagen base de Rust con Debian (Bullseye) para un entorno de compilación más estable.
FROM rust:1-bullseye AS builder

# Establecer el directorio de trabajo.
WORKDIR /usr/src/app

# Copiar todo el workspace.
COPY . .

# Copiar migrations al directorio del crate
RUN cp -r migrations services/identity-service/app/migrations

# Establecer SQLX_OFFLINE para usar queries cacheadas
ENV SQLX_OFFLINE=true

# Compilar el identity-service en modo release.
RUN cargo build --release --bin identity-service-app --target x86_64-unknown-linux-gnu

# --- Etapa 2: Runner ---
FROM debian:bookworm-slim AS final
RUN apt-get update && apt-get install -y curl && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-gnu/release/identity-service-app .
CMD ["./identity-service-app"]