fn main() {
    let bit = 1;
    if(bit == 1){
        println!("bit is true!");
    }
    let bit = 0;
    if(bit == 1){
        println!("bit is true!");
    }
    else{
        println!("bit is not true... false......!");
    }
    let bit = 2;
    if(bit == 1){
        println!("bit is true!");
    }
    else if(bit==0){
        println!("bit is not true... false......!");
    }
    else{
        println!("bit is... what???");
    }

    println!("ternary operator sample bit is 2? {}",if bit==2 {"yes!"} else {"no..."});

    let mut bit = 1;
    loop{
        println!("loop bit {}", bit);
        if bit==4{
            break;
        }
        bit = bit<<1;
    }

    let mut bit = 1;
    while bit!=16{
        println!("while bit is {}", bit);
        bit = bit<<1;
    }

    for i in (1..16){
        println!("{i}");
    }
}
