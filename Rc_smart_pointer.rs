// -------------------------------------------
// 		Reference Counting Pointers
         //where we need owners of a value we use the rest smart pointer called RC or reference
// -------------------------------------------
/* 
use std::rc::Rc; //it takes rc pointers in scope
enum List{
   //Cons(i32, Box<List>),
   
   //problem solve krnay kay liye box ki jga rc aa gya

   Cons(i32, Rc<List>),
   Nil,
}

use crate::List::{Cons, Nil};
fn main(){ 
   /* 
   let a = Cons(1, Box::new(Cons(2, Box::new(Nil)))); 
   let b = Cons(3, Box::new(a)); 
   let c = Cons(4, Box::new(a)); //here we will give error It says use of moved value of a second line pr hm nay 
   //a ko move kr diya hai b mai. ownership of List A has been transferred to list B. variable a does not exist  
   //anymore there for in third line it gives us error

   //--------------------let us see how this problem handle us Rc pointers--------------------- 
   */ 

   let a = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Nil))))); 
   println!("Count after creating a = {}", Rc::strong_count(&a));

   {
   let b = Rc::new(Cons(3, Rc::clone(&a))); //we use a.clone() but this is not convention it not give us error
   println!("Count after creating b = {}", Rc::strong_count(&a)); 

   let c = Rc::new(Cons(4, Rc::clone(&a))); 
   println!("Count after creating c = {}", Rc::strong_count(&a));
   }
   println!("Count after code block = {}", Rc::strong_count(&a)); 

   //Please note that we always need to call the clone function to tell rest that we are going to create
   //another owner which will be pointing to same data.
}
*/ 


use std::rc::Rc; 

fn make_rc() -> Rc<String> {
 
   let s1 = Rc::new(String::from("Hello")); 
   println!("Count when the pointer is created {}", Rc::strong_count(&s1));  

   let s2 = s1.clone(); 
   println!("Count after the clone is created for the pointer {}", Rc::strong_count(&s1)); 
   s2

   //At the end of the function, both of the RC pointers will be dropped.

} 

fn main(){

   //in the main we call the fun The function will create an RC pointer S1.
  // Next, we create another RC pointer s2 that points to the variable of s1.
   let s2 = make_rc(); 
   println!("Count after function call {}", Rc::strong_count(&s2));
}