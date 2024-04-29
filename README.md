# TestingApi

[![Argo CD Badge](https://argocd.nzdev.org/api/badge?name=testing-api&revision=true)](https://argocd.nzdev.org/applications/argocd/testing-api)
[![Build and Test](https://github.com/Notliam99/TestingApi/actions/workflows/dockerBuild.yaml/badge.svg)](https://github.com/Notliam99/TestingApi/actions/workflows/dockerBuild.yaml)
[![Rust](https://github.com/Notliam99/TestingApi/actions/workflows/compileAndTest.yaml/badge.svg)](https://github.com/Notliam99/TestingApi/actions/workflows/compileAndTest.yaml)
[<img src="https://img.shields.io/badge/DockerHub-Images-blue.svg?logo=docker">](https://hub.docker.com/r/asskit/testing_api)
[<img src="https://img.shields.io/badge/GHCR-Images-green.svg?logo=github">](https://github.com/Notliam99/TestingApi/pkgs/container/testingapi)

This api is a testing ground for our next digital project where we have to be adapted to this dynamic. In other words we are giving it a trial run. 👍🏿

## How To Use

[<img src="https://img.shields.io/badge/Prebuilt%20Binaries-29b9cc.svg" height="35">](https://github.com/Notliam99/TestingApi/releases/latest)
</br>

## Using source:

Make sure to download [RUSTUP](rustup.rs) before trying to run or build from source as the program is coded in [Rust](https://www.rust-lang.org/)

### Run from source immediately after building in debug mode. (Exports to: `target/debug`)
```zhs
cargo run
```


### Build from source in release mode. (Exports to: `target/release`)
```zhs
cargo build --release 
```

## Unit Testing:

### Run Unit Tests
```zhs
cargo test --package testing_api
```

