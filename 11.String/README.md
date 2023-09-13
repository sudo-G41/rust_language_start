# String
## rust String...
c language string is char arrays. rust to similar vector but no char is u8 vector(byte arrays) so rust string get index &ne; chars index
```rust
let asdf = String::new("ㅁㄴㅇㄹ");
let s = asdf[1..2];
println!("{}",s);
```

    thread 'main' panicked at 'byte index 1 is not a char boundary; it is inside 'ㅁ' (bytes 0..3) of `ㅁㄴㅇㄹ`', test.rs:3:14
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

but this code is success
```rust
fn main(){
    let asdf = String::from("asdf");
    let s = &asdf[1..2];
    println!("{:?}",s);
}
```
    "S"
## String is utf-8
utf-8 char is
* en = 1byte
* kr = 3byte(os, cpu, ... different)

so en string is index ok but different char is err

## you get char on index?
this fn use!
```rust
let asdf = String::from("ㅁㄴㅇㄹ");
let asdf = asdf.chars();//chars make
let asdf:Vec<char> = asdf.collect(); //chars to vector<char>
println!("{:?}", asdf[1]);
```
similar you want bytes? chars() => bytes() change!

## string split
rust string spilt solution
1. split(char)
1. split_at(int)
1. split_off(int)
1. slicing

first split(char) similar python split()!  
next split_at and split_off is cut! but return is different
split_at(int) : resut is tuple in cut index
split_off(int) : result is cut string and start byte
Ex)
```rust
fn main(){
    let qwer_asdf = "qwerasdf";
    let s = String::from(qwer_asdf);
    let (qwer,asdf) = s.split_at(4);
    println!("{:?} {:?}", qwer, asdf);
    let mut qwer = String::from(qwer_asdf);
    let asdf = qwer.split_off(4);
    println!("{:?} {:?}", qwer, asdf);
}
```
## String and &str
4. slicing! python string [.....] same  
&str to string slicing!!! so String and &str similar but not same!!

## and?end!
ye~~~~~s!:wq