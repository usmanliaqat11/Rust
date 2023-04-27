   // -------------------------------------------
   // 			Iterators     
   //           	- Basics
   //          		- Some useful functions for iterators
   //           	- Common statistics 
   // -------------------------------------------

   fn main() 
   {
       let some_vec = vec![1, 2, 3,4,5,6,7];
       let mut iter = some_vec.iter();

       println!("The iterator : {:?}", iter); 
       println!("{:?}",iter.next());
       println!("{:?}",iter.next());
       println!("{:?}",iter.next());
       println!("{:?}",iter.next());
       println!("{:?}",iter.next());
       println!("{:?}",iter.next());
       println!("{:?}",iter.next());
       println!("{:?}",iter.next());
   

   let a:Vec<u32> = vec![0,1,2,4,5,6,9,8,7];
       
   //let mut check = a.iter().any(|&x| x > 10); //koi aik condition satisfy honi chahiye
   //And inside the closure we need to mention a variable which will assume the values contained in the .   iterator one by one Since since this function operates on the values that are created using the dot iter,   which uses reference to the values itself are not the actual values.
   //println!("The value of the any fucntion is {}",check);
   
   let check = a.iter().all(|&x| x > 0); //mut is liye use nhi kiya kyu kay value change nhi ho rhi is line
   //of code main
   println!("The value of the all fucntion is {}",check);
   
   let check = a.iter().find(|&&x| x > 0);     
   //&&x explanation This is because, as pointed out before, the DOT title uses references, so the value is given to the function are behind reference. Moreover, the function .find also uses references to values, so we are required to use double references
  println!("The value of the  function find  is {}",check.unwrap()); //ist value jo bhi condition satisfy kry ge usy print kr day ga.
   
 //the dot any and dot find functions has some similarities, but they are a bit different
 //.any() return true or false while the .find() return the value itself  

    
    let check = a.iter().position(|&x| x > 5);  
    println!("The value of the  function position is {}",check.unwrap());
   
    let check = a.iter().rposition(|&x| x >8);  
   println!("The value of the function rposition is {}",check.unwrap());//right side say jo pehli value mily ge usko return kr day ga
//    
//    
//    let check = a.iter().max();     
//    println!("The value of the function max is {}",check.unwrap());
 
//    let check = a.iter().min();  
//    println!("The value of the function min is {}",check.unwrap());
 
//    let check:u32 = a.iter().sum();
//    let check:u32 = a.iter().product(); 
//    println!(" {:?}", check); 
//    
//    
//    let mut iter = a.iter().rev(); 
//    println!("The result of applying the rev fucntion {:?}", iter); //is trah reverse values nhi day ga ye fun // jb .next kry gay tb ye reverse kry ga
//    println!(" {:?}", a); 
}