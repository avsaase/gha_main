FROM rust:1.69 as build

# create a new empty shell project
RUN USER=root cargo new --bin rust-gh-action
WORKDIR /rust-gh-action

# copy your source tree
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src

# build for release (use sparse registry protocol)
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
RUN cargo build --release

# our final base
FROM gcr.io/distroless/cc AS runtime

# copy the build artifact from the build stage
COPY --from=build /rust-gh-action/target/release/rust-gh-action .

# set the startup command to run your binary
ENTRYPOINT ["/rust-gh-action"]