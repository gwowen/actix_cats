# h/t https://shaneutt.com/blog/rust-fast-small-docker-image-builds/
# for the original source of this
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
RUN apk update && apk upgrade && apk add bash
# we don't want to run as the root user, so create a group
# and user to run it as!
RUN addgroup -g 1000 actix_cats_user
RUN adduser -D -s /bin/sh -u 1000 -G actix_cats_user actix_cats_user
# copy actix_cats from build container along with static files
COPY --from=cargo-build /usr/local/cargo/bin/actix_cats /usr/local/bin/actix_cats
RUN mkdir static
COPY static ./static
# change ownership of app and static files
# to actix_cats_user 
RUN chown actix_cats_user:actix_cats_user /usr/local/bin/actix_cats
RUN chown -R actix_cats_user:actix_cats_user static
USER actix_cats_user
CMD [ "actix_cats" ]
EXPOSE 8080/tcp