fn main(){
    //sample
    let value = '@';//any value
    println!("...string...{}...", value);
    println!("");

    //align
    println!("{{:<#}} is left align #{:<5}#", '@');
    println!("{{:>#}} is right align #{:>5}#", '@');
    println!("{{:^#}} is center align #{:^5}#", '@');
    println!("{{:0align#}} is left, right or center align after space -> 0 change #{:0^5}#", '@');
    println!("");

    //Decimal to...
    println!("Dec 15 -> ...");
    println!("{{:b}} is binary number #{:b}#", 15);
    println!("{{:0#b}} is similar {{:>0#}} but binary values #{:08b}#", 15);
    println!("{{:o}} is binary number #{:o}#", 15);
    println!("{{:0#o}} is similar {{:>0#}} but octal values #{:03o}#", 15);
    println!("{{:x}} is binary number #{:x}#", 15);
    println!("{{:0#b}} is similar {{:>0#}} but hex values #{:01x}#", 15);
    println!("");

    //other #
    println!("{{:.#}} is floating number round # #pie is {:.3}#", 3.1415926535);
    println!("{{:e}} is exponential number #1000000 -> {:e}#", 1000000);
    println!("");

    //other format
    println!("{{{{, }}}} is {{, }}");
    println!("\\\", \\\\ is \\\", \\");
    println!("{{:p}} is value pointer(value address)");
    println!("");

    //debug print..?
    println!("{{:?}}, {{:#?}} is debug print(this # is not number, it's #) #?is align");
    println!("");

    //value index choice
    println!("{0}value, {2}value, {1}value,", 1, 2, 3);
    println!("");
}
