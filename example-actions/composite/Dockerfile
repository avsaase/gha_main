FROM rust:1.69 as build

# create a new empty shell project
RUN USER=root cargo new --bin rust-gh-action
WORKDIR /rust-gh-action

# Use sparse registry protocol
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/rust_gh_action*
RUN cargo build --release

# our final base
FROM gcr.io/distroless/cc AS runtime

# copy the build artifact from the build stage
COPY --from=build /rust-gh-action/target/release/rust-gh-action .

# set the startup command to run your binary
ENTRYPOINT ["/rust-gh-action"]