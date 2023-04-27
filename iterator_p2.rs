   // -------------------------------------------
   // 			Iterators     
   //           	- Basics
   //           	- Some useful functions for iterators
   //           	- Common statistics 
   //           	- Modifying and collecting values
   // -------------------------------------------

   fn main() 
   {
    let a = vec![0, 1, 2, 3,4,5,6,7];
  
   
    let filtered_values = a.iter().filter(|&x| *x >= 5).collect::<Vec<&u32>>(); 
    //The filter function is going to create an iterator which uses a closure to determine if an element should
    println!(" {:?}", filtered_values); //this is return refrences
  
   
    let b = a.clone();
    let filtered_values= a.into_iter().filter(|x| *x >= 5).collect::<Vec<u32>>(); 
    println!(" {:?}", filtered_values); //this is not return refrences

  // println!(" {:?}", a); 
   
   
    let mut mapped_values = b.iter().map(|x| 2 * *x).collect::<Vec<u32>>();    
    println!(" {:?}", mapped_values); 
   
   
   
    let mut mapped_values = b.iter().map(|x| 2 * x).filter(|x| *x > 10).collect::<Vec<u32>>();    
   
    println!(" {:?}", mapped_values); 
   
    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map 
   
   } 
   
   