fn main() {
    let x = 5;
    println!("The value of x is {}",x);
    //x = 6; is error variable is immutable
    let mut y = 5;
    println!("The value of y is {}",y);
    //y is mutaility
    y = 6;
    println!("The value of y is {}",y);
    //x is never change? nope rust shadowing is variable to change
    let x = 6;
    println!("The value of x is {}, y is {}", x, y);
}
