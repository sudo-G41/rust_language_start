mod img;

use img::Img;
use image;
use std::io::{stdin, stdout, Write};
use std::path;

pub fn run(){
    println!("2 colors choice");
    let idx = 0;
    loop{
        println!("1. RGB");
        println!("2. Hex");
        print!("input : ");
        stdout().flush().unwrap();

        let idx = match input_str().parse::<isize>(){
            Ok(i)   => i,
            Err(_)  => 0,
        };

        match idx{
            1|2 => break,
            _   => println!("plz 1 or 2"),
        };
    }
    let mut colors = [Img::NULL;2];
    colors[0] = input(&idx);
    colors[1] = input(&idx);
    let mut img_colors:Vec<image::Rgb::<u8>> = Vec::new();
    for i in 0..2{
        let (r,g,b) = colors[i].get_rgb();
        img_colors.push(image::Rgb::<u8>([r,g,b]));
    }
    let (px,width) = get_size();
    let draw = |x:u32,y:u32|{
        let (xi,yi) = (x/px, y/px);
        match (xi%2, yi%2){
            (0,0)|(1,1) => img_colors[0],
            (0,1)|(1,0) => img_colors[1],
            _           => panic!("draw err"),
        }
    };
    let img = image::ImageBuffer::from_fn(width,width,draw);
    let dir:String = String::new();
    loop{
        print!("image path : ");
        stdout().flush().unwrap();
        let dir = input_str();
        if let Ok(_) = path::PathBuf::from(&dir).read_dir(){
            break;
        }
        else{
            println!("path err");
        }
    }
    let dir:String/mnt/c/Users/sutjjang/Desktop/mnt/c/Users/sutjjang/Desktop = format!("{}/checker.png", dir, );
    img.save(&dir).unwrap();
}

fn get_size()->(u32,u32){
    let mut px:u32 = 0;
    let mut width:u32 = 0;
    loop{
        print!("px size : ");
        stdout().flush().unwrap();
        match input_str().parse::<u32>(){
            Ok(w)   => {
                px = w;
                break;
            },
            Err(_)  => println!("Integer plz"),
        };
    }
    loop{
        print!("checker count(width) : ");
        stdout().flush().unwrap();
        match input_str().parse::<u32>(){
            Ok(c)   => {
                width = c;
                break;
            },
            Err(_)  => println!("Integer plz"),
        };
    }
    let px:u32 = px;
    let width:u32 = px*width;
    (px, width)
}

fn input_str()->String{
    let mut s = String::new();
    stdin().read_line(&mut s).expect("Failed to read line");
    s.trim().to_string()
}

fn input_rgb()->Img{
    let mut red:u8 = 0;
    loop{
        print!("Red : ");
        stdout().flush().unwrap();
        match input_str().parse::<u8>(){
            Ok(r)   => {
                red = r;
                break;
            },
            Err(_)  => println!("Red is 0~255"),
        };
    }

    let mut green:u8 = 0;
    loop{
        print!("Green : ");
        stdout().flush().unwrap();
        match input_str().parse::<u8>(){
            Ok(g)   => {
                green = g;
                break;
            },
            Err(_)  => println!("Green is 0~255"),
        };
    }

    let mut blue:u8 = 0;
    loop{
        print!("Blue : ");
        stdout().flush().unwrap();
        match input_str().parse::<u8>(){
            Ok(b)   => {
                blue = b;
                break;
            },
            Err(_)  => println!("Blue is 0~255"),
        };
    }
    Img::from((red,green,blue))
}

fn input_hex()->Img{
    loop{
        print!("Hex : #");
        stdout().flush().unwrap();
        let code:String = input_str();
        if code.len() != 6{
            println!("Enter the 6-digit code");
            continue;
        }
        let parse_code = code.parse::<u32>();
        if let Ok(c) = parse_code{
            let hex = Img::from(c);
            if let Img::HEX(_) = hex{
                return hex;
            }
            else{
                println!("hex is 000000~ffffff({})", 0xffffff);
            }
        }
        else{
            let char_vec:Vec<char> = code.chars().collect();
            let mut hex:u32 = 0;
            for ch in char_vec{
                hex *= 0x10;
                hex += match ch{
                    '0'..='9'   => u32::from(ch)&15_u32,
                    'a'|'A'     => 0xA as u32,
                    'b'|'B'     => 0xB as u32,
                    'c'|'C'     => 0xC as u32,
                    'd'|'D'     => 0xD as u32,
                    'e'|'E'     => 0xE as u32,
                    'f'|'F'     => 0xF as u32,
                    _           => {
                        hex = 0x1000000 as u32;
                        break;
                    },
                };
            }
            let hex:Img = Img::from(hex);
            if let Img::HEX(_) = hex{
                return hex;
            }
            else{
                println!("hex is 000000~ffffff({})", 0xffffff);
            }
        }
    }
}

fn input(idx:&isize)->Img{
    if *idx == 1{
        return input_rgb();
    }
    else{
        return input_hex();
    }
}

#[cfg(test)]
mod img_tests{
    use super::*;
    #[test]
    fn rgb_from(){
        let img = Img::from((0,0,0));
        assert_eq!(img, Img::RGB(0,0,0));
        assert_ne!(img, Img::RGB(255,255,255));
        assert_ne!(img, Img::HEX(0xffffff));
    }
    #[test]
    fn hex_from(){
        let img = Img::from(0xffffff);
        assert_ne!(img, Img::RGB(0,0,0));
        assert_eq!(img, Img::RGB(255,255,255));
        assert_eq!(img, Img::HEX(0xffffff));
    }
    fn rgb_geter(){
        let img = Img::from(0xff009a);
        assert_eq!(img.get_r(),0xff);
        assert_eq!(img.get_g(),0x00);
        assert_eq!(img.get_b(),0x9a);
        assert_eq!(img.get_rgb(), (255,00,0x9a));
    }
}
