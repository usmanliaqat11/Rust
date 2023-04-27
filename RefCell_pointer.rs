// -------------------------------------------
// 		RefCell
         //it holds the sigle ownership over the data like box smart pointer
         //The difference is that the box smart pointer enfoce borrow rule comile time
         //Ref Cell smart pointer enfoce borrow rule at run time
// 		    - Borrowing rules checked at run time 
// 		    - Interior Mutablity
// 		    - Rc with RefCell

// -------------------------------------------


/*
use std::cell::RefCell;  

fn main()  
{  

    /*
    let mut x = 50; 
    let x1 = &x; 
    let x2 = &x; 
    let x3 = &mut x;

    println!("{} {} ", x1,x2);
   */

   /* 
    let a = RefCell::new(10); 
    //let b = a.borrow();  //line1
    //let c = a.borrow();//line2
    let d  = a.borrow_mut();
   //we already have borrowed the same value as immutable it gives us error at line1 and line2 

   The second point is that unlike ordinary variables, the borrower will last until the variable exit the 
//scope
//This means that the variable B, C, and D will remain until we exit from the scope or code segment
//This is different from ordinary variable cases where the scope starts from the point where they are
//end use

    //drop(b); 
    //drop(c); 
     let d  = a.borrow_mut();  
     drop(d);
     println!("Value of a is : {:?}",a);
     
 
    { 
    let b = a.borrow();  
    let c = a.borrow(); 
    }

    
    



    
    
    

    //drop(b); 
    //drop(c); 
    
    let d  = a.borrow_mut();  
    drop(d);
    println!("Value of a is : {:?}",a);

   */

   /* 
   let x = 32; 
   let x1 = &mut x; 
   */ 

 //yhi same kam hm refcell say kr sktay hai

   /*
   let a = RefCell::new(10);
   //let mut b = a.borrow_mut();   
   //  *b = 15;
   let *a.borrow_mut() = 15; 
   let mut b = a.borrow_mut();
   *b = 15;

   //drop(b); 
   println!("{:?}",a);



    */
  //} 

  */

  //let x=32
  //let y=&mut x
  
 





  use std::cell::RefCell;  
  use std::rc::Rc; 
  
  
   
  fn main()  
  {  
  
  
    let a = Rc::new(RefCell::new(String::from("java")));  
    let b  = Rc::clone(&a); 
    
    *b.borrow_mut() = String::from("c++"); 
    println!("{:?}",a); 
  
  }  
  
   