# Module
class, lib, is module?!
## How to use module
1. same code file add
1. other directory
1. workspace

## Same code file
```rust
pub mod a{
    pub fn function{
        ...
    }
}
fn main(){
    a::function();
}
```

mod module_name{
    ...
}

### Waht is pub
pub is public is mod(fn) is public mod(fn) you think C++ access modifier public

## Other directory
tree

    .
    ├── Cargo.lock
    ├── Cargo.toml
    └── src
        ├── module_name
        │   ├── new_file.rs
        │   ├── ...
        │   └── mod.rs
        ├── ...
        └── main.rs


new dir and this dir on mod.rs, new module file
```shell
    $ mkdir src/module_name
    $ touch src/module_name/mod.rs
    $ touch src/module_name/new_file.rs
    $ touch src/module_name/...
    $ ...
```

mod.rs is this module mod prototype write
```rust
pub mod new_file;
...
```

new_file.rs is mod code
```rust
pub fn function(){
    ...
}

pub mod sub_mod{
    ...
}
```

main.rs use directory `name::modulename`;
```rust
use module_name::new_file;

fn main(){
    new_file::function();
    new_file::sub::mod::...
    ...
}
```

## package? workspace!
    .
    ├── Cargo.toml
    ├── main
    │   ├── Cargo.toml
    │   └── src
    │       └── main.rs
    └── lib
        ├── Cargo.toml
        └── src
            └── lib.rs

main Cargo.toml write [dependencies]
```toml
...
[dependencies]
lid = {path = "lib real path(../lib)"}
...
```
./Cargo.toml write only [workspace]
```toml
[workspace]
members = [
    "main",
    "lib",
    ...
]
```
use is same
```rust
use lib::...;
```
members = [link members]
:wq