FROM rust:latest as builder

RUN USER=root cargo new --bin elonaire-backend
WORKDIR /elonaire-backend
COPY . .
RUN cargo build --release
RUN rm src/*.rs

FROM alpine:latest
WORKDIR /usr/src/app
EXPOSE 8000
COPY --from=builder /elonaire-backend/target/release/elonaire-backend /usr/src/app/elonaire-backend
CMD ["./elonaire-backend"]