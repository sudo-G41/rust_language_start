pub fn mod1_b_fn_a()->String{
    "mod dir in b fn a".to_string()
}

pub mod mod1_b_in_a{
    pub fn mod1_b_in_a_in_fna(){
        println!("super : {}", super::mod1_b_fn_a());
    }
}
