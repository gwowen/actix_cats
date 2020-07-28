FROM rust as cargo-build
# build and run inside the container
WORKDIR /usr/src/actix_cats
COPY . .
RUN cargo install --path .

FROM alpine:latest
COPY --from=cargo-build /usr/local/cargo/bin/actix_cats /usr/local/bin/actix_cats
COPY static .
CMD [ "actix_cats" ]
EXPOSE 8080/tcp