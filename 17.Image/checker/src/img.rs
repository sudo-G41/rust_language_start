use std::fmt;

#[derive(Debug)]
pub enum Img{
    RGB(u8,u8,u8),
    HEX(u32),
    NULL,
}

impl Img{
    pub fn get_r(&self)->u8{
        let code = self.get_hex();
        let code = code / 0x10000;
        code as u8
    }
    pub fn get_g(&self)->u8{
        let code = self.get_hex();
        let code = code / 0x100;
        let code = code % 0x100;
        code as u8
    }
    pub fn get_b(&self)->u8{
        let code = self.get_hex();
        let code = code % 0x100;
        code as u8
    }
    pub fn get_rgb(&self)->(u8,u8,u8){
        match self{
            Img::RGB(r,g,b) => (*r,*g,*b),
            Img::HEX(code)  => {
                let code = *code;
                let b = code%0x100;
                let code = code/0x100;
                let g = code%0x100;
                let r = code/0x100;
                (r as u8, g as u8, b as u8)
            },
            Img::NULL       => todo!(),
        }
    }
    pub fn get_hex(&self)->u32{
        match self{
            Img::RGB(r,g,b) => u32::from(*r)*0x10000+u32::from(*g)*0x100+u32::from(*b),
            Img::HEX(code)  => *code,
            Img::NULL       => todo!(),
        }
    }
}

impl From<(u8,u8,u8)> for Img{
    fn from(code:(u8,u8,u8))->Img{
        let (r, g, b) = code;
        Img::RGB(r,g,b)
    }
}

impl From<u32> for Img{
    fn from(code:u32)->Img{
        if code>0xffffff{
            return Img::NULL;
        }
        Img::HEX(code)
    }
}

impl PartialEq<Img> for Img{
    fn eq(&self, other:&Self)->bool{
        match (self, other){
            (Img::NULL, Img::NULL)  => true,
            _                       => self.get_hex() == other.get_hex(),
        }
    }
}

impl Copy for Img{}

impl Clone for Img{
    fn clone(&self)->Img{
        *self
    }
}

impl fmt::Display for Img{
    fn fmt(&self, f:&mut fmt::Formatter)->fmt::Result{
        match self{
            Img::RGB(r,g,b) => write!(f, "{}", format!("Red     [{:03}]\nGreen  [{:03}]\nblue   [{:03}]",*r, *g, *b)),
            Img::HEX(code)  => write!(f, "{}", format!("#{:06X}", *code)),
            Img::NULL       => write!(f, "This is not a color!"),
        }
    }
}
