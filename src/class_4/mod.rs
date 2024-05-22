mod conditions;
mod assignments;


use conditions::fn_controls;
use assignments::{fn_input,rev_print,input_sub};

pub fn mod_fn_controls(){
    fn_controls();
    println!();
    rev_print();
    println!();
    fn_input();
    println!();
    // input_sub();
}