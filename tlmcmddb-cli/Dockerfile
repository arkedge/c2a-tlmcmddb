FROM ghcr.io/sksat/cargo-chef-docker:1.70.0-bullseye as chef

WORKDIR /build

FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# build
FROM chef as builder
# build deps(cached)
COPY --from=planner /build/recipe.json recipe.json
RUN cargo chef cook --release -p tlmcmddb-cli --recipe-path recipe.json
# build bin
COPY . .
RUN cargo build --release -p tlmcmddb-cli

FROM debian:bullseye-slim AS runtime
WORKDIR /app
COPY --from=builder /build/target/release/tlmcmddb-cli /usr/local/bin
ENTRYPOINT ["/usr/local/bin/tlmcmddb-cli"]
