mod in_module{
    pub mod in_a{
        pub fn in_a_fn_a(){
            println!("in a fn a");
        }
        fn in_a_fn_b()->String{
            "in a fn b".to_string()
        }
        pub fn in_a_fn_c(){
            println!("in a fn c in [{}]", in_a_fn_b());
        }
        mod in_a{
            pub fn in_a_in_a_in_fna(){
                println!("ina ina fna");
            }
        }
        pub fn inaina(){
            in_a::in_a_in_a_in_fna();
        }
    }
    pub mod in_b{
        pub fn in_b_fn_a(){
            println!("in b fn a");
        }
    }
}

mod mod1;

use in_module::{in_a, in_b};
use crate::mod1::mod1a;
use mod1::mod1b;
use workspace;

fn main() {
    in_a::in_a_fn_a();
    in_a::in_a_fn_c();
    in_b::in_b_fn_a();
    in_a::inaina();
    mod1a::mod1_a_fn_a();
    mod1b::mod1_b_in_a::mod1_b_in_a_in_fna();
    workspace::workspace_fn_a();
}
