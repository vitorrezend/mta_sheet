# ==============================================================================
# MTA Sheet - Dockerfile Multi-Stage Ultra-Leve (~20MB)
# ==============================================================================

# ------------------------------------------------------------------------------
# Stage 1: Build (Rust Alpine Builder)
# ------------------------------------------------------------------------------
FROM rust:alpine3.20 AS builder

# Instala ferramentas essenciais de compilação
RUN apk add --no-cache musl-dev build-base pkgconfig openssl-dev curl

# Instala target WebAssembly
RUN rustup target add wasm32-unknown-unknown

# Instala CLI oficial do wasm-bindgen correspondente a versao do Cargo.lock
RUN cargo install wasm-bindgen-cli --version 0.2.121 --locked

WORKDIR /app

# Copia manifestos e codigo-fonte
COPY Cargo.toml Cargo.lock ./
COPY style.css ./
COPY styles ./styles
COPY src ./src
COPY tests ./tests

# 1. Compila Frontend WASM Release
RUN cargo build --lib --target wasm32-unknown-unknown --release --no-default-features --features hydrate

# 2. Gera bindings JS e empacota assets do site
RUN mkdir -p target/site/pkg && \
    wasm-bindgen --target web --out-dir target/site/pkg --out-name mta_sheet \
    target/wasm32-unknown-unknown/release/mta_sheet.wasm --no-typescript && \
    cp style.css target/site/pkg/mta_sheet.css

# 3. Compila Servidor Backend SSR Estatico (linkado com musl)
RUN cargo build --release --no-default-features --features ssr

# ------------------------------------------------------------------------------
# Stage 2: Runtime (Alpine Minimal Image ~20MB)
# ------------------------------------------------------------------------------
FROM alpine:3.20 AS runtime

# Instala certificados SSL, fuso horario e curl para healthcheck
RUN apk add --no-cache ca-certificates tzdata curl

# Cria usuario e grupo nao-root por seguranca
RUN addgroup -g 10001 -S appgroup && \
    adduser -u 10001 -S appuser -G appgroup

WORKDIR /app

# Cria pastas para o banco SQLite persistente e uploads de imagens
RUN mkdir -p /app/data /app/uploads /app/target/site/pkg /app/styles && \
    chown -R appuser:appgroup /app

# Copia binario compilado e assets estaticos do estagio de build
COPY --from=builder --chown=appuser:appgroup /app/target/release/mta_sheet_server /app/mta_sheet
COPY --from=builder --chown=appuser:appgroup /app/target/site /app/target/site
COPY --from=builder --chown=appuser:appgroup /app/style.css /app/style.css
COPY --from=builder --chown=appuser:appgroup /app/styles /app/styles

# Variaveis de ambiente padrao para containerizacao
ENV DATABASE_URL="sqlite:/app/data/mta_sheet.db?mode=rwc"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV RUST_LOG="info"

# Porta padrao da aplicacao
EXPOSE 3000

# Executa com usuario sem privilegios de root
USER appuser

# Healthcheck automatico
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:3000/ || exit 1

# Comando de inicializacao
CMD ["/app/mta_sheet"]
