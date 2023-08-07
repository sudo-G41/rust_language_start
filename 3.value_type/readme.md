# rust value type
## #type
### Number
Signed int : i# (# is bit 8~128)  
Unigned int : u# (# is bit 8~128)  
OS bit signed : isize  
OS bit unsigned : usize  
Floating number int : f# (# is bit 64, 32)  

### Text
character : char  
string : str  

## let or let mut
rust value is immutable only(let) but you want chang data value? add mut  
``` rust
Ex)  
let a = 10  
a = a + 1 <-Error
let mut a = 10
a = a + 1 <- success
```

## arrays
rust arr : let (mut) arr_name = [start value; size];

initial value arr : let (mut) arr_name = [value, val, ....];

rust arr index start : 0

