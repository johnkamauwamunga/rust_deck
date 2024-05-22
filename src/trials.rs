mod fruits;
//mod others;
mod animals;

mod utils{pub mod core;}

use utils::core::fn_slices;

use animals::hello_mod;
//use others::fn_hello;

fn main() {
// print_fruits();
// best_fruits();
//  animals::hello_mod();
// utils::core::core_fun();
//core_fun();

// a list of noms

let v=vec![10,20,30,40];

print_vector(&v);

println!("print the actual vector {:?}",v);

//hello_mod();

let mut val1=3;

mutate(&mut val1);

print!("updated value {}",val1);

println!();

fn_slices();
}

fn print_vector(x:&Vec<i32>){
    println!("inside print vector fun {:?}", x);
}

// updatting borrowed value by mutating 

fn mutate(e:&mut i32){
    *e+=1
}