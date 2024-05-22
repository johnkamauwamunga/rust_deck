struct User{
    name: string,
    age:i32,
    id_number:i32,
}

impl User{
    pub fn walk(&self){
        println!("user {} is walking");
    }
}