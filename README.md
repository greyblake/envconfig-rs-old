# Envconfig

An easy way to build a config structure form environment variables in Rust.

## Usage

Let's say you application relies on the following environment variables:

* `DB_HOST`
* `DB_PORT`

And you want to initialize `Config` structure like this one:

```rs
struct Config {
  host: String,
  port: u16
}
```

You can achieve this with the following code without boilerplate:

```rs
#[macro_use] extern crate envconfig;

envconfig!(Config {
    db_host: String = "DB_HOST",
    db_port: u16 = "DB_PORT"
});


let config = Config::init_or_die();
```

## Running tests

To prevent flaky tests run them in one thread:

```
cargo test -- --test-threads=1
```
