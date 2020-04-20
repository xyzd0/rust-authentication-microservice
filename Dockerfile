####################################################################################################
# create a builder stage, due to SQLx compile time query checks, this requires an active postgres
# connection. we will set this up first.
FROM rust:1.42 AS builder
WORKDIR /app/src

# install dependencies
RUN apt-get update
RUN apt-get install -y clang libclang-dev
RUN rustup component add rustfmt

# copy Cargo.toml and create a dummy program to ensure build dependencies are cached
COPY server/Cargo.toml .
RUN mkdir -p src
RUN echo "fn main() { panic!(\"build broken\") }" > src/main.rs
RUN cargo build --release
RUN rm -rf src/
RUN rm -rf target/release/authentication*
RUN rm -rf target/release/deps/authentication*

# copy full source and build
COPY proto ../proto
COPY server .
RUN cargo build --release

####################################################################################################
# create the runner stage. this is a fresh image which takes the compiled binary from the builder
# stage and runs it without any additional dependencies required.
FROM ubuntu:18.04
WORKDIR /app/bin
COPY --from=builder /app/src/target/release/authentication .
ENTRYPOINT ["/app/bin/authentication"]
