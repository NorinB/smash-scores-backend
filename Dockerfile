FROM rust:1.65 as build

# create a new empty shell project
RUN USER=root cargo new backend
WORKDIR /backend

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies due to docker layering
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/smash_scores_backend*
# RUN cargo build --release --offline
RUN cargo build --release

# our final base
FROM debian:buster-slim

# copy the build artifact from the build stage
COPY --from=build /backend/target/release/smash-scores-backend /usr/src/smash-scores-backend

# set the startup command to run your binary
CMD ["/usr/src/smash-scores-backend"]