# Use the official Rust image as the builder
FROM rust:latest as builder

# Set the working directory
WORKDIR /usr/src/straico-client

# Copy the entire project to the container
COPY . .

# Build the application
RUN cargo build --release

# Use a smaller image for the final stage
FROM debian:buster-slim

# Set the working directory
WORKDIR /usr/local/bin

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/straico-client/target/release/straico-proxy .

# Expose the port the app runs on
EXPOSE 8000

# Command to run the application
CMD ["./straico-proxy"]
