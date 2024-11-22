# Build stage
FROM rust:slim-bookworm AS builder

# Install system dependencies for OpenSSL and pkg-config
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install dioxus-cli
RUN cargo install --git https://github.com/DioxusLabs/dioxus dioxus-cli --locked

# Set the working directory
WORKDIR /usr/src/app

# Copy the entire project
COPY . .

# Build the project
RUN dx build --release

# Final stage
FROM debian:bookworm-slim

# Install necessary dependencies for running the server
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy the entire web directory from the builder stage
COPY --from=builder /usr/src/app/target/dx/DrawsNotes/release/web ./

ENV PORT=8080
ENV IP=0.0.0.0
# Expose the port your server listens on (adjust if necessary)
EXPOSE 8080

# Run the server
CMD ["./server"]
