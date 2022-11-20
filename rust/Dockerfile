FROM lukemathwalker/cargo-chef:latest AS chef
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release 



FROM gcr.io/distroless/cc
WORKDIR /app
COPY --from=builder /app/target/release/smart_bot ./
EXPOSE 8001
CMD ["/app/smart_bot"]