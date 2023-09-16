enum IpAddr{
    V4(u8, u8, u8, u8),
    V6(u16, u16, u16, u16, u16, u16, u16, u16),
    None,
}

impl IpAddr{
    fn get_ip(&self)->String{
        match*self{
            IpAddr::V4(a,b,c,d)         => format!("{}.{}.{}.{}", a, b, c, d),
            IpAddr::V6(a,b,c,d,e,f,g,h) => format!("{:04x}:{:04x}:{:04x}:{:04x}:{:04x}:{:04x}:{:04x}:{:04x}", a,b,c,d,e,f,g,h),
            IpAddr::None                => String::from("None..."),
        }
    }
}

enum Op<T>{
    S(T),
    NULL,
}

fn main() {
    let localhost:IpAddr = IpAddr::V4(127,0,0,1);
    let loopback:IpAddr = IpAddr::V6(0,0,0,0,0,0,0,1);
    println!("{}",localhost.get_ip());
    println!("{}",loopback.get_ip());
    let loopback = IpAddr::V6(0x2001,0x0DB8,0x1000,0x0000,0x0000,0x0000,0x1111,0x2222);
    println!("{}",loopback.get_ip());
    let asdf:Op<&str> = Op::S("qwer");
    match asdf{
        Op::S(v)    => {
            println!("{}",v);
        },
        Op::NULL    => {
            println!("Is't null");
        }
    };
    let asdf = Op::<&str>::NULL;
    match asdf{
        Op::S(v)    => {
            println!("{}",v);
        },
        Op::NULL    => {
            println!("Is't null");
        }
    };
}
