# syntax=docker/dockerfile:1
FROM rustlang/rust:nightly AS build

# Install WASM target and cargo-leptos
RUN rustup target add wasm32-unknown-unknown --toolchain nightly
RUN cargo install cargo-leptos

# Copy the project
COPY . /project

# Build the server and client
WORKDIR /project
RUN cargo leptos build --release

# Start the runtime container
FROM ubuntu:latest AS runtime

# Copy the server binary
COPY --from=build project/target/server/release /
COPY --from=build project/target/site /site

ENV LEPTOS_OUTPUT_NAME="recipe-book"
ENV LEPTOS_SITE_ROOT="site"
ENV LEPTOS_SITE_PKG_DIR="pkg"
ENV LEPTOS_SITE_ADDR="127.0.0.1:3000"
ENV LEPTOS_RELOAD_PORT="3001"

# Expose the port and run the server
EXPOSE 3000
ENTRYPOINT ["./recipe-book"]
