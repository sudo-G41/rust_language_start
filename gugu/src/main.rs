fn main() {
	for i in 1..10{
		for j in 1..10{
			print!("{:4}{}",i*j, if j!=9{","}else{""});
		}
		print!("\n");
	}
}
