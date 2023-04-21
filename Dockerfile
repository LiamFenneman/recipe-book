# syntax=docker/dockerfile:1
FROM rustlang/rust:nightly-bullseye as builder

# Install WASM target and cargo-leptos
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN cp cargo-binstall /usr/local/cargo/bin 
RUN cargo binstall cargo-leptos -y
RUN rustup target add wasm32-unknown-unknown

# Copy and build the project
RUN mkdir -p /app
WORKDIR /app
COPY . .
RUN cargo leptos build --release -vv
RUN ls -l /app/target

# Start the runtime container
FROM rustlang/rust:nightly-bullseye as runner

# Copy the server binary
COPY --from=builder /app/target/server/release/recipe_book /app/
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/Cargo.toml /app/

# Run the server
WORKDIR /app
ENV LEPTOS_OUTPUT_NAME="recipe_book"
ENV APP_ENVIRONMENT="production"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 8080
ENTRYPOINT ["/app/recipe_book"]
