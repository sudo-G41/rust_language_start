# From trait
struct or enum construct fn is from  
but from(T) <- T type match different body?  
rust is not function overloading
## From trait fn from
but trait is similar inheritance  
```rust
	pub enum Img{
		RGB(u8,u8,u8),
		HEX(u32),
		None,
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
```

From<type> impl for struct or enum ... and fn from make :w
