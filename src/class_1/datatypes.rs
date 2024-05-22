pub fn fn_datatypes(){
// integers

let score:i16=129; //overflow 2


println!("the scode {}", score);

let a: i32 =-3;

println!("value of z {}", a);
// booleans


// muttable and immutable

let mut fees:i32=25_000;

fees=35_000;

println!("the current fees is {}",fees);

// static variables

let name:&'static str= "john";

println!("my name is {}",name);

// creating tuples

let tuple: (&str, f32, u8)=("rust",3.14, 100);

println!("tuple content: {:?}", tuple); //al the tuple elements

// print single item

println!("tuple 0 is {:?}", tuple.0);
println!("tuple 1 is {:?}", tuple.1);
println!("tuple 2 is {:?}", tuple.2);

//creating a mutable tuple

let mut mountains= ("Everest", 8855, "kilimsnjaro", 15011);

//output original tuple

println!("origin a unchanged tuples {:?}", mountains);

//change the value of one tuple

mountains.0="Kenya";
mountains.1=5199;

println!("after updatin tuple {:?}", mountains);

// destructuring a tuple\

let tup=("john doe", 18, 175);
// destructure using below dataset

let(name, age, height)=tup;


//print destructured data

println!("tuple name {:?}", name);
println!("tuple age {:?}", age);
println!("tuple height {:?}", height);
}