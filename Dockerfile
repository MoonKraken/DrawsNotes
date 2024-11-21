# Build stage
FROM rust:alpine AS builder

# Install build dependencies and dioxus-cli
# Install build dependencies, dioxus-cli, and static OpenSSL libraries
RUN apk add --no-cache musl-dev git openssl-dev openssl-libs-static

# Set environment variables for static linking
ENV OPENSSL_STATIC=1
ENV OPENSSL_DIR=/usr
RUN cargo install --git https://github.com/DioxusLabs/dioxus dioxus-cli --locked

# Set the working directory
WORKDIR /usr/src/app

# Copy the entire project
COPY . .

# Build the project
RUN dx build --release

# Final stage
FROM alpine:latest

# Install necessary dependencies for running the server
RUN apk add --no-cache ca-certificates

# Set the working directory
WORKDIR /app

# Copy the entire web directory from the builder stage
COPY --from=builder /usr/src/app/target/dx/DrawsNotes/release/web ./

# Expose the port your server listens on (adjust if necessary)
EXPOSE 8080

# Run the server
CMD ["./server"]
