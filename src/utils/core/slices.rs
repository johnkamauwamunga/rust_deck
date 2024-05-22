//used to return a portion of data stored in a continous memory is stack, vector, string.
// uses index to fetchyb the data from the mem location

pub fn slicing(){
 let n1="CaisyKhalif".to_string();


 println!();
 println!("the length of string is {}", n1.len());

 let c1=&n1[3..9];

 println!("sliced portion: {}",c1);


 let arr=[1,2,3,4,5,6];

 let arr1=&arr[2..5];

 println!();
 
 println!("the sliced nueric data is {:?}",arr1);

}