use rand::Rng;

fn main() {
    println!("Lotto # create");
    let mut num_list = [0;44];
    let mut rng = rand::thread_rng();

    for i in 1..=43{
        num_list[i] = i+1;
    }

    for i in 1..=43{
        let swap_idx = rng.gen_range(1..=43);
        num_list[0] = num_list[swap_idx];
        num_list[swap_idx] = num_list[i];
        num_list[i] = num_list[0];
    }
    println!("{:?}", &num_list[1..=7]);
}
