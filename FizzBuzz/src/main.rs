fn main() {
	let mut i = 0;
	let mut f = true;
	while i!=100{
		i += 1;
		f = true;
		if i%3 == 0{
			print!("Fizz");
			f = false;
		}
		if i%5 == 0{
			print!("Buzz");
			f = false;
		}
		if f{
			print!("{}",i);
		}
		print!("\n");
	}
}
