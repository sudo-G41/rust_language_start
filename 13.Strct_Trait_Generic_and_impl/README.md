# Strct, trait, Generic and impl
## Generic
generic is any type Ok
Ex)
```rust
fn function<GenericT>(t:GenericT){
    println!("{}", std::any::type_name::<GenericT>());
}

fn main(){
    function(String::new());
    function("");
    function(1);
    function(1.0);
    function(vec!['_']);
    function([1]);
    function(Vec::<u8>::new());
}
```
    alloc::string::String
    &str
    i32
    f64
    alloc::vec::Vec<char>
    [i32; 1]
    alloc::vec::Vec<u8>

function parameter type name is GenericT but rust not have GenericT type.  
main call function() and argument is String, integer, char, array, vector multiplex type.  
call function return argument print  
so generic is any type parameter is ok change argument type!
## how to use generic
    <generic name>
Ex)
```rust
//T is generic
fn function<T>->Vec<T>{...}
```

## Strct
```rust
struct NAME<generic>{
    val name:type,
    val name:type,
    ...
}
```
struct is user make new type  
C/C++ language struct similar  
## struct construct
example struct Book, Book have pages, name
```rust
struct Book{
    name    : String,
    pages   : usize,
}
```
new Book make momo
```rust
let momo = Book{
    name    : String::from("momo"),
    pages   : 367
}
```
or use fn
```rust
fn new_Book(name:String, pages:usize){
    Book{
        name    : name,
        pages   : pages,
    }
}
```
get value
```rust
println!("name:{}, total pages:{}", momo.name, momo.pages);
```

## Trait
Defining Shared Behavior similar abstract class but only fn  
```rust
trait NAME<generic>{
    fn function(parameter:type)->return type;
    ...
}
```
or
```rust
trait NAME<generic>{
    fn function(parameter:type)->return type{
        ...
    }
    ...
}
```
this success
```rust
trait NAME<generic>{
    fn function(parameter:type)->return type;
    fn function(parameter:type)->return type{
        ...
    }
    ...
}
```

## Inheritance? no but similar impl
### Struct method
we create handun struct
```rust
struct Handgun{
    model   : String,
    bullet  : usize,
    max     : usize,
}
```
you wnat shot and reload fn 
```rust
fn shot(gun:mut Handgun)->(bool,Handgun){
    let result = match gun.bullet{
        0   => {
            println!("plz reload");
            false
        },
        _   => {
            println!("shot!");
            gun.bullet -= 1;
            true
        },
    }
    (result,gun)
}
fn reload(gun:mut Handgun)->Handgun{
    gun.bullet = gun.max;
    gun
}
```
maybe you make many fn and refactory shot? search hard and handgun fn counting is hard so now use impl
```rust
impl Handgun{
    fn shot(&mut self)->bool{
        let result = match self.bullet{
            0   => {
                println!("plz reload");
                false
            },
            _   => {
                println!("shot!");
                self.bullet -= 1;
                true
            },
        }
        result
    }
    fn reload(&mut self){
        self.bullet = self.max;
    }
}
```
## Trait and struct chain
now this code upgrade  
shot and reload is gun fn  
gun is hg, ar, rf, sg, smg... we struct this!(hg, ar)
```rust
struct HG{
    model   : String,
    bullet  : usize,
    max     : usize, 
}
struct AR{
    model   : String,
    mode    : usize,
    bullet  : usize,
    max     : usize,
}

trait Gun{
    fn shot(&mut self)->bool;
    fn reload(&mut self);
}
```
Hg chain gun and AR chain gun
```rust
impl Gun for HG{
    fn shot(&mut self)->bool{
        let result = match self.bullet{
            0   => {
                println!("{} reload plz", self.model);
                false
            },
            _   => {
                print!("{} shot!", self.model);
                self.bullet -= 1;
                println!(" [{}]", self.bullet);
                true
            },
        };
        result
    }
    fn reload(&mut self){
        println!("{} reload", self.model);
        self.bullet = self.max;
    }
}

impl Gun for AR{
    fn shot(&mut self)->bool{
        let result = match self.bullet{
            0                   => {
                println!("{} reload plz", self.model);
                false
            },
            n if n<self.mode    => {
                print!("{} shot!", self.model);
                self.bullet = 0;
                println!(" [{}]", self.bullet);
                true
            },
            _                   => {
                print!("{} shot!", self.model);
                self.bullet -= self.mode;
                println!(" [{}]", self.bullet);
                true
            },
        };
        result
    }
    fn reload(&mut self){
        println!("{} reload", self.model);
        self.bullet = self.max;
    }
}
```
HG is only 1 shot but AR is mode diffrent
```rust
fn main(){
    let mut hoxy = HG{
        model:"Five-seveN".to_string(),
        bullet:5,
        max:30,
    };
    let mut k2 = AR{
        model:"K-2".to_string(),
        bullet:5,
        max:30,
        mode:1,
    };
    let mut g11 = AR{
        model:"G11".to_string(),
        bullet:15,
        max:30,
        mode:3,
    };
    hoxy.shot();
    hoxy.shot();
    k2.shot();
    hoxy.shot();
    hoxy.shot();
    hoxy.shot();
    g11.shot();
    g11.shot();
    k2.shot();
    k2.shot();
    hoxy.shot();
    k2.shot();
    k2.shot();
    k2.reload();
    k2.shot();
}
```

    Five-seveN shot! [4]
    Five-seveN shot! [3]
    K-2 shot! [4]
    Five-seveN shot! [2]
    Five-seveN shot! [1]
    Five-seveN shot! [0]
    G11 shot! [12]
    G11 shot! [9]
    K-2 shot! [3]
    K-2 shot! [2]
    Five-seveN reload plz
    K-2 shot! [1]
    K-2 shot! [0]
    K-2 reload
    K-2 shot! [29]