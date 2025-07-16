FROM rust:1-slim-bookworm AS builder

WORKDIR /app

# Copy Cargo.toml and Cargo.lock to leverage Docker cache
COPY Cargo.toml Cargo.lock ./

# Build dependencies, but not the application itself yet
# This step will fail if src is not present, but it's good for caching dependencies
# RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release --target x86_64-unknown-linux-gnu
# The above is not strictly necessary if src is copied right after, as cargo build --release will handle it.
# However, for pure dependency caching, it's a common pattern.
# For this specific case, where there are no external dependencies beyond std,
# copying src and building directly is fine.

# Copy the source code
COPY src ./src

# Build the application in release mode
RUN cargo build --release

# --- Start a new stage for the final, smaller image ---
FROM debian:bookworm-slim

WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/IMT-rust-codelabs ./IMT-rust-codelabs

# Set the entry point to run the compiled application
CMD ["./IMT-rust-codelabs"]