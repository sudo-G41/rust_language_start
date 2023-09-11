use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args:Vec<String> = env::args().collect();
    let args = &args[1..];
    if args.len() < 1{
        println!("[file name] find word1 find word2...");
        return;
    }

    let file_name = &args[0];
    let args = &args[1..];

    for word in args{
        let fp = match File::open(file_name){
            Ok(f) => f,
            Err(e) => {
                println!("[{}] file open Err : {}",file_name, e);
                std::process::exit(1);
            },
        };
        println!("find word : {}",word);
        find_word_in_file(&word, &fp);
        println!("");
    }
}

fn find_word_in_file(word:&str, fp:&File){
    let mut idx = 0;
    let reader = BufReader::new(fp);
    for line in reader.lines(){
        idx += 1;
        match line{
            Ok(s) => {
                if s.find(word) != None{
                    println!("{} line : {}", idx, s);
                }
            },
            Err(e) => {
                println!("find err : {}", e.to_string());
            },
        };
    }
}
