use std::{env, path, any};

fn main() {
    let path = get_path();
    tree(&path, 0);
}

fn tree(path:&path::PathBuf, level:usize){
    let file = match path.read_dir(){
        Ok(f)   => f,
        Err(e)  => {
            println!("\"{}\" is Err[{}]", path.display(), e.to_string());
            return;
        }
    };
    for ent in file{
        let path = match ent{
            Ok(e)   => e.path(),
            Err(e)  => {
                println!("Err[{}]", e.to_string());
                return;
            }
        };

        for _ in 1..=level{
            print!("│   ");
        }

        let fname = path.file_name().unwrap()
                    .to_string_lossy();
        if path.is_dir(){
            println!("├─ [{}]", fname);
            tree(&path,level+1);
        }
        else{
            println!("├── {}", fname);
        }
    }
}

fn get_path()->path::PathBuf{
    let path:Vec<String> = env::args().collect();
    
    let path = match path.len(){
        1   => ".",
        _   => &path[1],
    };
    path::PathBuf::from(path)
}
