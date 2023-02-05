fn main() {
    println!("Hello, world!");
    another_function();
    another_function2(1);
    another_function3(2, "is bit?");
    another_function4();
    println!("7++?:{}",another_function5(7));
}

fn another_function(){
    println!("?!!!!!!");
}

fn another_function2(x:i32){
    println!("x:{x}");
}

fn another_function3(x:i32, s:& str){
    println!("x:{x}, str:{s}");
}

fn another_function4(){
    // let x = (let y = 4) is Error
    // x = y = value can not this code
    let x = 4;
    let y = {
        let y = 3;
        y+1
    };
    println!("y:{y}");
}

fn another_function5(x:i32) -> i32{
    x + 1
    //can "return x+1;" code
}
