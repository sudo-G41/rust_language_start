# Input
## std io!
rust input is std::io use
```rust
use std::io::stdin;
stdin().read_line(value);
```
but it's only String! you want other type? plz parse()...  
Ex)
```rust
let mut s = String::new();
stdin().read_line(&mut s).expect("Failed to read");

let i = s.trim().parse::<i32>();
match i{
	Ok(r) => {
	    return r;
	},
	Err(e) => {
	    println!("Err! {:?}", e);
	}
}
```
# match
macth similar switch in C language  
Ex)
```c
int num = 1;
switch num{
    case 1:
	...
	break;
    case 2:
	...
	break;
    default:
	...
	break; //생략 가능
}
```
```rust
let num = 1;
match num{
    1 => ... ,
    2 => ... ,
    _ => ...
};
```
switch is only Integer compare but rust is various types ex enum...

it's enum days study! ㅎㅎ...
