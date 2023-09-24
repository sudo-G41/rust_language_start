pub mod image{
    use std::fmt;
    pub enum Img{
        RGB(u8,u8,u8),
        HEX(u32),
        None,
    }

    impl fmt::Display for Img{
        fn fmt(&self, f: &mut fmt::Formatter)->fmt::Result{
            match self{
                Img::RGB(r,g,b) => write!(f, "{}", format!("Red[{:03}]\nGreen[{:03}]\nBlue[{:03}]", r, g, b)),
                Img::HEX(code)  => write!(f, "Hex[#{:06x}]", code),
                Img::None       => write!(f, "this is not color"),
            }
        }
    }

    impl From<(u8,u8,u8)> for Img{
        fn from(code:(u8,u8,u8))->Img{
            let (r,g,b) = code;
            Img::RGB(r,g,b)
        }
    }

    impl From<u32> for Img{
        fn from(code:u32)->Img{
            if code > 0xffffff{
                println!("Hex code is ffffff or less");
                return Img::None;
            }
            Img::HEX(code)
        }
    }

}

fn main() {
    let img = image::Img::from(0xff0fff);
    println!("{}",img);
    let img = image::Img::from((0,0,0));
    println!("{}",img);
    let img:image::Img = (0xff,255,0xff).into();
    println!("{}",img);
    let img = image::Img::from(0x1000000);
    println!("{}",img);
}

