fn main() {
    println!("Rust Date type!");
    println!("first Integer!");
    let x = 8;
    println!("{} bit i{}, u{}", x, x, x);
    let x = 16;
    println!("{} bit i{}, u{}", x, x, x);
    let x = 32;
    println!("{} bit i{}, u{}", x, x, x);
    let x = 64;
    println!("{} bit i{}, u{}", x, x, x);
    let x = 128;
    println!("{} bit i{}, u{}", x, x, x);
    println!("arch is computer bit type isize, usize");
    println!("i is signed, u is unsigned");
    println!("DHOBB(Decimal, Hex, Octal, Binary, Byte[Byte is u8 only])");
    println!("D(98_222):{}, H(0xff):{}, O(0o77):{}, B(0b1111_0000):{}, B(b'A'):{}", 98_222, 0xff, 0o77, 0b1111_0000, b'A');

    println!("floating point f64 and f32");

    println!("bool is true or false");

    println!("char is alpabetic type and emoji");

    println!("tup is () we can destructuring or (.)followed");

    let arr = [[0;5];5];
    println!("arr is array! yes array! {:?} {:#?}", arr[0], arr[1]);
}
