# HashMap
## HashMap is ...
HashMap is [key,valeu] datatype..?
similar c++ map, python dict ...
## How to use?
```rust
use std::collections::HashMap;

let name_vlaue:HashMap<_,_> = HashMap::new()!!
```
_ is you want datatype  
Ex)
```rust
let dict:HashMap<String, String> = HashMap::new();

let name_age:HashMap<String, u8> = HashMap::new();

let rank_id:HashMap<usize, String> = HashMap::new();
...
```
insert is
```rust
HashMap.insert(data,data2);
```
insert fn use!!

HashMap get result <Some,None>
- Some(v) : v in HashMap key
- None => not in Key...

## owner...
Ex)
```rust
let name_age:HashMap<String, i32> = HashMap::new();
let name = "qwerty_see";
name_age.insert(name,-1234);
```
after use name? nope! name owner is HashMap!....?!!!  
so name life is end..? life in HashMap?
so HashMap<&str,_> <= be careful! insert data life cycle end Err!

### ****
소유권 문제+라이프사이클덕에 예제코드 만드는데 오래 걸려버렸구만... 끔찍하다..
이런점이 빡세네 러스트... 하지만 그만큼 메모리누수 없는건 느껴지네.......