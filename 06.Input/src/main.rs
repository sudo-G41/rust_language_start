use rand::Rng;
use std::io::stdin;
use std::io::{self,Write};

fn main() {
    let com = rand::thread_rng().gen_range(1..=3);
    match com{
        1..=3 =>{}
        _ => panic!("******g com!")
    }
    let player = input("1.가위 2.바위 3.보 : ");
    let player = s2i(&player);
    println!("com {} you {}", what(com), what(player));
    let winer = match who_win(com, player){
        0 => "무승부".to_string(),
        1 => "승리".to_string(),
        -1 => "패배".to_string(),
        _ => "??????? hey w*** *** ****?".to_string()
    };
    println!("{}!",winer);
}

fn input(s:&str)->String{
    print!("{}",s);
    io::stdout().flush().unwrap();
    let mut result = String::new();
    // stdin
    stdin().read_line(&mut result).expect("Failed to read");
    result
}

fn s2i(s:&str)->i32{
    let result = s.trim().parse::<i32>();
    match result{
        Ok(r) => {
            return r;
        },
        Err(e) => {
            println!("Err! {:?}", e);
        }
    }
    -1
}

fn who_win(p1:i32, p2:i32)->i8{
    if p1 == p2{
        return 0;
    }
    match p2{
        1 =>{
            if p1 == 2{return -1}
            else{return 1}
        },
        2 =>{
            if p1 == 3{return -1;}
            else{return 1}
        },
        3 =>{
            if p1 == 1{return -1;}
            else{return 1}
        },
        _ => return -2
    }
}

fn what(n:i32)->String{
    match n{
        1 => "가위".to_string(),
        2 => "바위".to_string(),
        3 => "보".to_string(),
        _ => "????".to_string()
    }
}
