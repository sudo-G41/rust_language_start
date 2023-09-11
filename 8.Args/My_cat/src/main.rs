use std::env;
use std::fs;

fn main() {
    let args:Vec<String> = env::args().collect();
    if args.len() < 2{
        println!("Empty!");
        return;
    }
    let path = &args[0];
    let args = &args[1..];

    for (idx, filename) in args.iter().enumerate(){
        println!("[{}.{}]", idx, filename);
        let text = match fs::read_to_string(filename){
            Ok(v) => v,
            Err(e) => e.to_string(),
        };
        println!("{}", text);
    }
}
