// match statement

pub fn fn_controls(){
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

// loops 

}