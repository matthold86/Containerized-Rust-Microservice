# First stage: Build the application
FROM rust:latest as builder

# Set the working directory in the container
WORKDIR /usr/src/app

# Copy the source code into the container
COPY . .

# Build dependencies
RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*
RUN cargo build --release

# Copy the source code and build the application
COPY src ./src
RUN cargo build --release

# Second stage: Create a smaller image
FROM debian:buster-slim

# Set the working directory in the container
WORKDIR /usr/src/app

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/actix_app .

# Expose the port on which your Actix web service listens
EXPOSE 8080

# Command to run the binary when the container starts
CMD ["./actix_app"]
