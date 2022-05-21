# Docker images comparison example

Simple rest apis returning hello world in port 8080 or `${server.port}`

## Comparison

| REPOSITORY       | SIZE   | BUILDTIME |
| ---------------- | ------ | --------- |
| rust-image       | 6.7MB  | 469s      |
| go-image         | 13.9MB | 2.1s      |
| springboot-image | 286MB  | 14.3s     |

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
