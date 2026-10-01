FROM rust

RUN apt-get update && \
  apt-get -y upgrade && \
  apt-get -y install libpq-dev

WORKDIR /app
COPY .. /app/
#COPY .env /app/.env
#COPY migrations /app/migrations

RUN cargo build --release

EXPOSE 8080

ENTRYPOINT ["/bin/bash", "-c", "cargo run --release"]