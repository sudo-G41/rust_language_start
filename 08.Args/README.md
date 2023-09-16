# Args, file read&write and if let
## Args
args in std::env
```rust
std:end::args();
```

this change vector
```rust
let v:Vec<String> = std:end::args().collect();
```
args[0] is now app path  
args[1..] is args

## File!
### read
1. use std::fs
fs::read_to_string(file_name)  
this file all text read  
Ok => String(file text all)

2. use io bufreader
file read before file open next read. this read a line  
```rust
use std::io::{BufRead, BufReader};
use std::fs::File;

let reader = BufReader::new(File::open(file name));
for s in reader.lines(){
	//read line is s
}
```
but File open need error try catch

### write before file create
File::create(file name);  
this new File make  
you think python 'w+' not 'a'  

### write
1. write(bufWrite)
BufWriter::new(file_pointer);  
this buf.write(data) but data is bytes so need string.as_bytes()
```rust
let fp = File::create(file name).unwrap();
let wb = BufWriter::new(fp);

wb.write("string".as_bytes()).unwrap();
```
2. write all(fs)
fs in function write_all() si file write all
```rust
let fp = File::create(file name).unwrap();
fp.write_all("string".as_bytes()).unwrap();
```

## if let...?
if let is error try catch control  
match is Ok(r) => {...}, Err(e) => {...} but if let is Ok(r) {...} else {...} or Err(e){...} else{...}
```rust
match wb.write("string".as_bytes()){
	Ok(r) => {...},
	Err(e) => {...},
}

if let Ok(r) = wb.write("string".as_bytes()){
	...
}
else{
	...
}
```
if let similar  
match ...{  
	Ok(r)	=> {...},  
	_			=> {...},  
}  
???

#### 아 힘들다..
힘드네...
에러 제어는 match, if let말고도  
unwrap(), unwrap_or(), unwrap_or_else()가 있지만 난 그냥 match쓸래...