
pub struct Rectangle{
   pub width: i32,
    pub length: i32,
}

impl Rectangle{
    pub fn area(&self) -> i32{
     return self.width * self.length;
    }
}