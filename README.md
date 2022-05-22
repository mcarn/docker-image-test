# Docker images comparison example

Simple rest apis returning hello world in port 8080 or `${server.port}`

## Comparison

| REPOSITORY       | SIZE   | BUILDTIME |
| ---------------- | ------ | --------- |
| rust-image       | 6.7MB  | 469s      |
| go-image         | 13.9MB | 2.1s      |
| node-image       | 75.5MB | 70.4s     |
| springboot-image | 286MB  | 14.3s     |

## Rust

```bash
$ docker build -t rust-image rust-image/
```

## Go

```bash
$ docker build -t go-image go-image/
```

## Node

```bash
$ docker build -t node-image .
```

## Java Spring Boot

```bash
$ mvn clean install
$ docker build -t springboot-image springboot-image/
```
