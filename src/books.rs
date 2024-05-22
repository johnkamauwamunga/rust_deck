pub struct Book{
    pub name: String,
    pub status: bool,
    pub category:String
}

impl Book{
 pub fn check_availability(&self) -> bool{
  return self.status
 }
}