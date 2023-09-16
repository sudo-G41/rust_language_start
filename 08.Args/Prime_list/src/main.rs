use std::io::stdin;
use std::env;
use std::fs::{self, File};
use std::io::{Write, BufWriter};

fn main() {
    let num = input_usize("Prime max num : ");
    let mut pList = prime_list(num);
    let s = vec2str(&pList);
    //one line write
    let file_name = "prime_list1.txt";
    let fp = match File::create(file_name){
        Ok(f) => f,
        Err(e) => {
            println!("File1 create Err [{}]", e.to_string());
            std::process::exit(-1);
        },
    };
    let mut wb = BufWriter::new(fp);
    for i in &pList{
        let mut bytes = i.to_string();
        bytes.push('\n');
        let bytes = bytes.as_bytes();
        if let Err(e) = wb.write(bytes){
            println!("line write Err [{}]", e.to_string());
            std::process::exit(-1);
        }
        //else{ ... }
    }

    //line all write
    let file_name = "prime_list2.txt";
    let mut fp = match File::create(file_name){
        Ok(f) => f,
        Err(e) => {
            println!("File2 create Err [{}]", e.to_string());
            std::process::exit(-1);
        },
    };
    let s = vec2str(&pList);
    let s = s.as_bytes();
    if let Err(e) = fp.write_all(s){
        println!("line all write Err [{}]", e.to_string());
        std::process::exit(-1);
    }
}

fn vec2str(v:&Vec<usize>)->String{
    let mut s = String::new();
    for t in v{
        s.push_str(&t.to_string());
        s.push('\n');
    }
    let s = s;
    s
}

fn input_usize(s:&str)->usize{
    print!("{}",s);
    std::io::stdout().flush().unwrap();
    let mut num = String::new();
    stdin().read_line(&mut num).expect("Failed to read");

    let num = num.trim().parse::<usize>();

    match num{
        Ok(r) => {
            return r;
        },
        Err(e) => {
            println!("Err! {:?}", e);
            std::process::exit(-1);
        }
    };
}

fn prime(num:usize)->bool{
    match num{
        2|3|5|7|11 => {return true;},
        _ => {},
    };
    match num%10{
        2|4|5|6|8|0 => {return false;},
        _ => {},
    };
    match num%12{
        1|5|7|11 =>{
            let mut i = 3;
            while i*i<num{
                if num%i == 0{
                    return false;
                }
                i += 2;
            }
            return num != i*i;
        },
        _ => return false,
    };
    false
}

fn prime_list(num:usize)->Vec<usize>{
    let mut pList:Vec<usize> = Vec::new();
    match num{
        1 => {
            println!("Empty... don't make list");
            std::process::exit(0);
        },
        2 => {
            pList.push(2);
        },
        _ => {
            pList.push(2);
            for i in 3..=num{
                if prime(i){
                    pList.push(i);
                }
            }
        },
    };
    pList
}
