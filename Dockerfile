FROM ubuntu:latest
COPY ./target/x86_64-unknown-linux-musl/release/rocket_blog_backend ./app
COPY ./.env ./.env
CMD ["./app"]