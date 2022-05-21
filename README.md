# Docker images comparison example

Simple rest apis returning hello world in port 8080 or ${server.port}

## Comparison

| REPOSITORY       | TAG    | SIZE   | BUILDTIME |
| ---------------- | ------ | ------ | --------- |
| rust-image       | latest | 6.7MB  | 469s      |
| go-image         | latest | 13.9MB | 2.1s      |
| springboot-image | latest | 286MB  | 14.3s     |

## Rust

```bash
$ docker build -t rust-image rust-image/
```

## Go

```bash
$ docker build -t go-image go-image/
```

## Java Spring Boot

```bash
$ mvn clean install
$ docker build -t springboot-image springboot-image/
```
