# --- Etapa 1: Builder ---
# Usamos una imagen base de Rust con Debian (Bullseye) para un entorno de compilación más estable.
FROM rust:1-bullseye AS builder

# Establecer el directorio de trabajo.
WORKDIR /usr/src/app

# Copiar todo el workspace.
COPY . .

# Compilar el portal-service en modo release.
RUN cargo build --release --bin portal-service-app --target x86_64-unknown-linux-gnu

# --- Etapa 2: Runner ---
FROM debian:bookworm-slim AS final
RUN apt-get update && apt-get install -y curl && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-gnu/release/portal-service-app .

# Establecemos un directorio de trabajo limpio.
WORKDIR /app

# Copiar el binario compilado desde la etapa de builder.
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-gnu/release/portal-service-app ./portal-service-app

# ¡Paso Clave! Copiar la carpeta de archivos estáticos.
# La copiamos al directorio de trabajo actual.
COPY --from=builder /usr/src/app/services/portal-service/app/static static

# Comando para ejecutar el servicio.
# El ejecutable ahora está en el WORKDIR.
CMD ["./portal-service-app"]