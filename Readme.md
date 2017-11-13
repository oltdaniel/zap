# `zap` :zap:

[![GitHub issues](https://img.shields.io/github/issues/oldaniel/zap.svg)](https://github.com/oldaniel/zap/issues)
[![GitHub stars](https://img.shields.io/github/stars/oldaniel/zap.svg?style=social&label=Stars)](https://github.com/oldaniel/zap)
[![Crates.io](https://img.shields.io/crates/d/zap.svg)](https://crates.io/crates/zap)
[![Crates.io](https://img.shields.io/crates/v/zap.svg)](https://crates.io/crates/zap)

The mission of `zap` is, to deliver a basic, but fast rust web framework.

[Documentation](https://docs.rs/zap/)

## About

**This code is based on tokio's minihttp project, so a big thanks to them.** ([source](https://github.com/tokio-rs/tokio-minihttp))

## How to use

Add the following to your `Cargo.toml`:
```toml
[dependencies]
zap = "0.0.4"
```

## Speed

So `zap` is not only fast, it is wapping **2.96** times faster than [iron](https://github.com/iron/iron), which is based on [hyper](https://github.com/hyperium/hyper). Benchmarks below:

### Benchmark Code

**Iron**

This code had been taken from the [ironframework.io](http://ironframework.io) webpage.

```rust
extern crate iron;

use iron::prelude::*;
use iron::status;

fn main() {
    fn hello_world(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello World!")))
    }

    Iron::new(hello_world).http("localhost:3000").unwrap();
}
```

**Zap**

This example can be run, by:

```
$ git clone https://github.com/oldaniel/zap && cd zap
$ cargo run --example hello-world --release
```

```rust
extern crate zap;

use std::io::Error as ZapError;
use zap::prelude::*;

struct HelloWorld;

impl Handler for HelloWorld {
    type Request = Request;
    type Response = Response;
    type Error = ZapError;
    type Future = ZapResult;

    fn call(&self, _: Request) -> ZapResult {
        let mut resp = Response::new();

        resp.body("Hello World!");

        resp.ok()
    }
}

fn main() {
    let addr = "0.0.0.0:8080".parse().unwrap();
    let mut server = Server::new(Http, addr);
    server.threads(8);
    server.serve(|| Ok(HelloWorld));
}
```

### Benchmark Results

The benchmark results have been computed with this command: `wrk -t16 -c500 -d10s http://127.0.0.1:8080 --latency`

Technical details about the server:

- Intel Core I7-6700K, hyper-threaded
- 16GB RAM, 2400MHZ

Detailed results: [in the wiki](https://github.com/oldaniel/zap/wiki/Benchmarks).

**Iron**

```
[...]
Requests/sec: 307581.17
Transfer/sec:     33.44MB
```

**Zap**

```
[...]
Requests/sec: 912832.31
Transfer/sec:     40.90MB
```

## Todo

Some things we still need to make:

- [X] Make it faster
- [ ] URL and body parsing
- [ ] Request key storing

## Credits & License

A project by [Daniel Oltmanns](https://github.com/oldaniel).
Other amazing contributors [here](https://github.com/oldaniel/zap/graphs/contributors).

[![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/oltmannsdaniel/zap/master/LICENSE)
