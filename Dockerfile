# build image
FROM rust:slim as builder

# update crates.io index, download, and build dependencies using a dummy binary
WORKDIR /usr/src/app/velocity
COPY velocity/Cargo.toml .
RUN mkdir -p src/bin && echo "fn main(){}" > src/bin/dummy.rs
RUN cargo build --release --bin dummy

# copy, build, and install the app
COPY velocity/ .
COPY assets/ /usr/src/app/assets/
RUN cargo build --release

# runtime image
FROM debian:bullseye-slim

# copy executable
WORKDIR /usr/src/app/velocity
RUN mkdir -p target/release/
COPY --from=builder /usr/src/app/velocity/target/release/velocity target/release/velocity

# run service
EXPOSE 8080
CMD ["./target/release/velocity"]
