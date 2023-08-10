use num_bigint::BigInt;

fn main() {
    let val = BigInt::from(1234);
    println!("1234 pow 5678 : {}", val.pow(5678));
}