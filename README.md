# Docker images comparison example

Simple rest apis returning hello world in port 8080 or ${server.port}

## Rust

```bash
$ docker build -t rust-image rust-image/
```

docker build: 403.8s

```dockerfile
####################################################################################################
## Builder
####################################################################################################
FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

# Create appuser
ENV USER=myapp
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /myapp

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release

####################################################################################################
## Final image
####################################################################################################
FROM scratch

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /myapp

# Copy our build
COPY --from=builder /myapp/target/x86_64-unknown-linux-musl/release/myapp ./

# Use an unprivileged user.
USER myapp:myapp

CMD ["/myapp/myapp"]
```

## Go

```bash
$ docker build -t go-image go-image/
```

docker build :7.7s

```dockerfile
FROM golang:1.13 as builder

WORKDIR /app
COPY . /app
RUN CGO_ENABLED=0 GOOS=linux GOPROXY=https://proxy.golang.org go build -o app main.go

FROM alpine:latest
# mailcap adds mime detection and ca-certificates help with TLS (basic stuff)
RUN apk --no-cache add ca-certificates mailcap && addgroup -S app && adduser -S app -G app
USER app
WORKDIR /app
COPY --from=builder /app/app .
ENTRYPOINT ["./app"]
```

## Java Spring Boot

```bash
$ mvn clean install
$ docker build -t springboot-image springboot-image/
```

maven build: 20.724 s
docker build : 7.7s
total: 28.5

```dockerfile
FROM eclipse-temurin:17-jre-focal

# Refer to Maven build -> finalName
ARG JAR_FILE=target/app.jar

# cd /opt/app
WORKDIR /opt/app

# cp target/spring-boot-web.jar /opt/app/app.jar
COPY ${JAR_FILE} app.jar

# java -jar /opt/app/app.jar
ENTRYPOINT ["java","-jar","app.jar"]
```

| REPOSITORY       | TAG    | SIZE   |
| ---------------- | ------ | ------ |
| rust-image       | latest | 6.7MB  |
| go-image         | latest | 13.9MB |
| springboot-image | latest | 286MB  |
