# rust-rest-api
Tutorial project to learn about developing RESTful API with Rust, using threading, smart-pointers, db querying, io etc.

## Badges
[![GitHub license](https://img.shields.io/github/license/c0de4un/rust-rest-api)](https://github.com/c0de4un/rust-rest-api/blob/main/LICENSE)
[![GitHub stars](https://img.shields.io/github/stars/c0de4un/rust-rest-api)](https://github.com/c0de4un/rust-rest-api/stargazers)

## Installation
* Just call cargo
```sh
    cargo run
```

## DataBase
Before testing or running, `db` must be enabled
```
    cd db
    docker-compose up --build -d
```
Check access `http://localhost:8181/`

## Testing
```sh
    cargo check
    cargo test
```

## Build
* Debug
```sh
    cargo build
```
* Release
```sh
    cargo build --release
```

## Usage
Start `Debug`-version
```
    ./target/debug/rust-rest-api
```

Start `Release`-version
```
    ./target/release/rust-rest-api
```

By default, server is running at `8080` port.
When application is running, open `localhost:8080`.
