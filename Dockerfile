# build image
FROM rust:slim as builder

# update crates.io index, download, and build dependencies
WORKDIR /usr/src/app
COPY jay/Cargo.toml .
RUN mkdir src && echo "fn main(){}" > src/main.rs
RUN cargo build --release

# copy, build, and install the app
COPY jay/ .
RUN cargo install --path .

# runtime image
FROM debian:bullseye-slim

# copy executable
WORKDIR /usr/src/app
COPY --from=builder /usr/local/cargo/bin/jay /usr/src/app/jay

# run service
EXPOSE 8080
# No CMD or ENTRYPOINT, see fly.toml with `cmd` override.
# CMD ["./jay"]
