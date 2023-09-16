# Enum
Enum is enumerated type but C/C++ enum is different
## Waht is different?
C/C++ is const data, rust is data type
```cpp
enum Date{
    Sun=0,
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat
}
```
```rust
enum IpAddr{
    V4(u8,u8,u8,u8),
    V6(u16,u16,u16,u16,u16,u16,u16,u16),
}
```
example C/C++ enum Sun = 0, Mon = 1 ... const data name but rust is IpAddr::V4 is u8 * 4 tuple data

    C/C++ enum  = constant data bundle
    Rust enum   = data type bundle

## Rust have not null but None
Rust have not null but Option::None on enum
```rust
enum Option<_>{
    Some(_),
    None
}
```
Option::None is i'm null similar
:w