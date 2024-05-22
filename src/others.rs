

fn main() {
    //println!("Hello, world!");
let num1=11_000.555_001;
a
println!("float num1 {}",num1);


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

// functions
// matchy statement

let state_code="GA";
let state=match  state_code{
    "MH"=>{
        println!("founmd match for  MH");
        "MAHARASHTRA"
    },
    "kl"=>"kERIA",
    "GA"=>"Goa",
    _=>"Unknown"
};
println!("state name is {}", state);

println!();
println!("pi value is {}:",fn_hello());

let no:i32= 5;
mutate_no(no);
println!("the value passed is {}",no);


// arrays in rust
//  declaraiopn
//  let variable=[val1,val2,val3] or let var_name:[datatype; size]=[val1, val2, val3];  or let var:[datatype; size]=[data_value_for_the_elements, size];

let arr:[i32;4]=[10,20,30,40];
println!();
println!("the array value {:?}", arr);

println!("array size is :{}", arr.len());

println!();
// loop to iterate and print values

let arrlen= arr.len();

for index in 0..arrlen{
 println!("index  is {} and value is {}",index,arr[index]);
}

println!();

for val in arr.iter(){
    println!("value is {}",val);
   }


// handling ownership


let v=vec![1,2,3]; // vector v owms the object in heap

//Only a single variable owns the heap memory at a time 
let v2=v; //here two variables owns heap vallue

//two pointers to the same content is not allowed in rust
// after the transfer thye v is invalidated.
println!();
println!(" value of v {:?}", v2);


// handling borrowing

}


pub fn fn_hello()->f64{
    22.0/7.0
}

fn mutate_no(mut param_no: i32){
    param_no=param_no*0;

    println!("param no is {}",param_no);
}