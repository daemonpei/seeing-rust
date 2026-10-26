# Two stages: a fat builder, a thin runtime.
FROM rust:1.82 AS build
WORKDIR /src

# Copy the manifests first so the dependency layer is cached across source edits.
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release --locked

# The runtime image has no compiler and no package manager.
FROM debian:bookworm-slim
RUN useradd -r -u 10001 app
COPY --from=build /src/target/release/ch34-release /usr/local/bin/ch34-release
USER 10001
ENTRYPOINT ["/usr/local/bin/ch34-release"]
