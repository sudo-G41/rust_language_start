use std::io::stdin;
use std::io::{self, Write};
use std::collections::HashMap;

fn main() {
    println!("what's this????");
    let mut dict:HashMap<String,(u16,u16,u16)> = HashMap::new();
    //dict.insert("su!",(010,1234,5678));
    loop{
        match input_char("add?(Y/N) : "){
            'Y' | 'y' => {
                let (name, phone) = add();
                if phone.0<10000{
                    dict.insert(name, phone);
                    print(&dict);
                }
            },
            'N' | 'n' => {
                if exit(){
                    break;
                }
            },
            _ => println!("????????????????")
        }
    }
}

fn input(s:&str)->String{
    print!("{}",s);
    io::stdout().flush().unwrap();
    let mut result = String::new();
    stdin().read_line(&mut result).expect("Failed to read");
    result
}

fn input_char(s:&str)->char{
    let c = input(s);
    let c = c.as_bytes()[0];
    char::from(c)
}

fn s2i(s:&str)->u16{
    let result = s.trim().parse::<u16>();
    match result{
        Ok(r) => {
            return r;
        },
        Err(e) => {
            println!("11111 Err! {:?}", e);
        }
    }
    11111
}

fn input_phone()->(u16,u16,u16){
    let phone = input("phone : ");
    let phone = phone.trim();
    let phone = match phone.len(){
        11 => {
            (&phone[0..3], &phone[3..7], &phone[7..11])
        },
        13 => {
            (&phone[0..3], &phone[4..8], &phone[9..13])
        },
        _ => {
            println!("????? phone numb.....er????");
            ("None", "NULL", "_")
        },
    };
    let result = phone.0.parse::<u16>();
    match result{
        Ok(_) => {
            return (s2i(phone.0), s2i(phone.1), s2i(phone.2));
        },
        Err(e) => {
            println!("19999 Err! {:?}", e);
            return (19999,19999,19999);
        }
    }
}

fn add()->(String, (u16,u16,u16)){
    let name = input("name : ").to_string();
    let name = name.trim();
    let phone = input_phone();
    (name.to_string(), phone)
}

fn print(dict:&HashMap<String,(u16,u16,u16)>){
    println!("======================================================");
    println!("{:^20} : phone #","name");
    println!("------------------------------------------------------");
    for (k,v) in dict{
        println!("{:^20} : {:03}-{:04}-{:04}", k, v.0, v.1, v.2);
    }
    println!("======================================================");
}

fn exit()->bool{
    let result = match input_char("quit? (Y/N) : "){
        'Y' | 'y' => true,
        'N' | 'n' => false,
        _ => {
            println!("??????? not exit????");
            false
        },
    };
    result
}
