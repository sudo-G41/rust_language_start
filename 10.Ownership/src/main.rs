fn main() {
    let mut a = 1;
    let mut b = 2;
    println!("a:{}, b:{}",a,b);
    swap(&mut a, &mut b);
    println!("a:{}, b:{}",a,b);

    let mut s = String::from("?");
    println!("s:{}",s);
    push1(&mut s);
    println!("s:{}",s);
}

fn swap(a:&mut isize, b:&mut isize){
    let c = *b;
    *b = *a;
    *a = c;
}

fn push1(s:&mut String){
    s.push('!');
}
