# syntax=docker/dockerfile:1

# --- Build Stage ---
# Pin the bookworm-based chef image so the built binary's glibc matches the
# bookworm-slim / distroless-debian12 runtime bases (latest builds on trixie).
FROM lukemathwalker/cargo-chef:latest-rust-1-bookworm AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json




FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release

# --- Runtime Targets ---

# 1. runtime-slim (Debugging image with shell)
FROM debian:bookworm-slim AS runtime-slim
# Patch base OS packages so image scanners (e.g. Trivy) see fixed CVEs
RUN apt-get update && apt-get -y upgrade && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/rust-template /app/rust-template
RUN groupadd -r nonroot && useradd -r -g nonroot nonroot
USER nonroot
EXPOSE 8080
HEALTHCHECK --interval=30s --timeout=3s CMD ["./rust-template", "--health-probe"]
CMD ["./rust-template"]

# 2. runtime-alpine (Minimal footprint)
# WARNING: Alpine uses musl libc. You MUST build the binary for the musl target (e.g., x86_64-unknown-linux-musl) to avoid runtime linkage failures.
FROM alpine:latest AS runtime-alpine
# Patch base OS packages so image scanners (e.g. Trivy) see fixed CVEs
RUN apk update && apk upgrade --no-cache
WORKDIR /app
# Note: This requires the binary to be built for musl.
# For a template, we'll assume a separate musl build stage or use a static binary.
COPY --from=builder /app/target/release/rust-template /app/rust-template
RUN addgroup -S nonroot && adduser -S nonroot -G nonroot
USER nonroot
EXPOSE 8080
HEALTHCHECK --interval=30s --timeout=3s CMD ["./rust-template", "--health-probe"]
CMD ["./rust-template"]

# 3. runtime-distroless (Hardened production image)
FROM gcr.io/distroless/cc-debian12 AS runtime-distroless
WORKDIR /app
COPY --from=builder /app/target/release/rust-template /app/rust-template
USER nonroot
EXPOSE 8080
HEALTHCHECK --interval=30s --timeout=3s CMD ["./rust-template", "--health-probe"]
CMD ["./rust-template"]
