FROM rust:1.75-slim as builder

WORKDIR /app
COPY . .

RUN cargo build --release -p {{project_name}}-cli

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/{{project_name}}-cli /usr/local/bin/{{project_name}}

EXPOSE 8080

ENTRYPOINT ["{{project_name}}"]
