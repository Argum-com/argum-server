FROM rust:1.73-alpine as builder

WORKDIR /app

# Install the latest version of the Rust toolchain.
RUN apk update && \
    apk upgrade

# Install the musl target so we can build static binaries.
RUN apk add --no-cache musl-dev

COPY Cargo.* ./
COPY src ./src

# Build the application.
RUN cargo build --release

FROM rust:1.73-alpine
ARG SERVER_PORT=3000
ENV PORT=$SERVER_PORT

WORKDIR /app

# Create a non-privileged user that the app will run under.
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/app" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser
# Copy the binary from the builder stage.
COPY --from=builder /app/target/release/argum-server .

# Expose the port that the application listens on.
EXPOSE ${PORT}

ENTRYPOINT [ "./argum-server" ]