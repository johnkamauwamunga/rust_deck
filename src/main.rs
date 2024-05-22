// manage books, each book has a nanme, status, company

// using structs.. implement

mod books;
mod rectangle;
mod class_1;
mod class_2;
mod class_4;

//use books::Book;


use rectangle::Rectangle;
use class_1::mod_datatypes;
use class_4::mod_fn_controls;



fn main(){


    let rect= Rectangle{
        length:8,
        width:7,
    };

    print!("area of rect is {}",rect.area());

   println!();

    println!("------- class 1---------");
      mod_datatypes();

      println!();
      println!("------- class 2---------");

      let num2=20;

      class_2::mutate_no(num2);
      let p=class_2::fn_hello();

      println!("pi value is {}:",p);

      println!();
      println!("------- class 4---------");
      mod_fn_controls();

    //   collect user inputs

    
}


// question, use struct to create a program to calculate the area of a rectangle/
// research on document attribute, previous assignment should contain documentation ,
//shoukd be able to runn cargo doc for that implememntation.