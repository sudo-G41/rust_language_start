use std::env;
use std::fs;

fn main() {
    let args:Vec<String> = env::args().collect();
    let args = &args[1..];
    if args.len() < 2{
        println!("?? plz more file name....");
        return;
    }
    else if args.len() > 2{
        println!("!! max file name is 2!!!");
        return;
    }

    let file0 = match fs::read_to_string(&args[0]){
        Ok(v) => (true,v),
        Err(e) => (false,e.to_string()),
    };
    let file1 = match fs::read_to_string(&args[1]){
        Ok(v) => (true,v),
        Err(e) => (false,e.to_string()),
    };

    if !file0.0{
        println!("{} file is {}", args[0], file0.1);
    }
    if !file1.0{
        println!("{} file is {}", args[1], file0.1);
    }
    if !(file0.0 && file1.0) {
        return;
    }

    let file0 = file0.1.trim();
    let file1 = file1.1.trim();
    if file0 == file1{
        println!("Ok!");
    }
    else{
        println!("Ng...");
    }
}
