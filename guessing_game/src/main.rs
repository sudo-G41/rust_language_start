extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("게임을 시작해보지");

    let A = rand::thread_rng().gen_range(1,101);

    println!("A:{}", A);
    loop{
        let mut q:String = String::new();
        println!("값을 입력해 봐라.");
        io::stdin().read_line(&mut q).
            expect("fail read line");
        let q:i32 = match q.trim().parse(){
            Ok(int) => int,
            Err(_)  => {
                println!("plz input Integer");
                continue;
            },
        };
        print!("you Q:{} ", q);
        match A.cmp(&q){
            Ordering::Less      => println!("DOWN?"),
            Ordering::Greater   => println!("UP?"),
            Ordering::Equal     => {
                println!("Nice!");
                break;
            }
        }
    }
}
