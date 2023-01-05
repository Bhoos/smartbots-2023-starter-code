FROM rust:1.66.0 as build-env
WORKDIR /app
COPY . /app
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=build-env /app/target/release/a-29-from-scratch smart_bot

EXPOSE 8001/tcp
CMD ["./smart_bot"]
