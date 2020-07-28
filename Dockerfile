# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------
FROM rust as cargo-build
# build inside the container
# first update and install musl-tools
RUN apt-get update
RUN apt-get install musl-tools -y
# add musl as a target
RUN rustup target add x86_64-unknown-linux-musl
WORKDIR /usr/src/actix_cats
COPY . .
# build against musl as a target
RUN RUSTFLAGS=-Clinker=musl-gcc cargo install --path . --target=x86_64-unknown-linux-musl

# ------------------------------------------------------------------------------
# Final Image Build Stage
# ------------------------------------------------------------------------------
FROM alpine:latest
# copy actix_cats from build container along with static files
COPY --from=cargo-build /usr/local/cargo/bin/actix_cats /usr/local/bin/actix_cats
COPY static .
CMD [ "actix_cats" ]
EXPOSE 8080/tcp