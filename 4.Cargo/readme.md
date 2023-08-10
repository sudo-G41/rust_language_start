# Cargo
-------
---------------------------------------------------
|Cargo help |Commend|
|-|-|
|make project|cargo new project_name|
|project build|cargo build or cargo b|
|project run|cargo run|
|analyze the current package and report errors, but don't build object files|cargo check or cargo c|
|project test|cargo test or cargo t|
|cargo docs make|cargo doc or cargo d|
|cargo package and upload this package to the registry|cargo publish|


any cmd search cargo --help

## cargo build, run
run need build file so run before build but don't fix code build only one you want build cmd build

so build --release cmd is release version build

## cargo toml file
this is cargo manifest file link : [Toml kr docs](https://toml.io/ko)

##Cargo example


make rust project
```sh
$ cargo new big_power
$ tree

.
└── big_power
    ├── Cargo.toml
    └── src
        └── main.rs
```

rust code
```rust
fn main() {
    let val:u128 = 1234;
    println!("1234 pow 5678 : {}", val.pow(5678));
}
```

this code is error
```sh
thread 'main' panicked at 'attempt to multiply with overflow', /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/num/mod.rs:936:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
u128 max size is 340282366920938463463374607431768211455(&ap;2<sup>128</sup>)

include num-bigint lib

```toml
[package]
name = "big_power"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
num-bigint = "0.4" # new
```

rust code
```rust
use num_bigint::BigInt;

fn main() {
    let val = BigInt::from(1234);
    println!("1234 pow 5678 : {}", val.pow(5678));
}
```
```sh
$ cargo run
...
1234 pow 5678 : 3072392956...
```