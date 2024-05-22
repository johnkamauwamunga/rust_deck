use std::io;

pub fn fn_input(){

   println!("enter the value of a");

    let mut a=String::new();

    io::stdin().read_line(&mut a).unwrap();
  
    let _a:i32= a.trim().parse().unwrap();

    let _b= _a*2;

    println!("your number is {}",_b);
}

// check input and subtract backwards
pub fn input_sub(){

    println!("enter the value of to loop back");
 
     let mut a=String::new();
 
     io::stdin().read_line(&mut a).unwrap();
   
     let _a:i32= a.trim().parse().unwrap();
 
     for _a in _a..=0{
        println!("{} ",_a)
     }
 
     
 }

// print values in reverse using range

pub fn rev_print(){
    println!("print values in reverse using range");
    for number in (1..=50).rev(){
        println!("{} ",number)
    }
}