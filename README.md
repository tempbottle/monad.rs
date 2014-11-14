# monad.rs

Stackless monads in Rust

[![build status](https://api.travis-ci.org/epsilonz/monad.rs.svg?branch=master)](https://travis-ci.org/epsilonz/monad.rs)

The `mdo!` macro is based on upon [rust-mdo](https://github.com/TeXitoi/rust-mdo) but modified to work with the different structures here.

## Documentation

See the API documentation [here](http://www.rust-ci.org/epsilonz/monad.rs/doc/monad/).

## Requirements

1.   [Rust](http://www.rust-lang.org/)
2.   [Cargo](http://crates.io/)

You can install both with the following:

```
$ curl -s https://static.rust-lang.org/rustup.sh | sudo sh
```

See [Installing Rust](http://doc.rust-lang.org/guide.html#installing-rust) for further details.

## Usage

```
$ cargo build       ## build library and binary
$ cargo test        ## run tests in ./tests
$ cargo bench       ## run benchmarks in ./benches
```
