/*

--------------------------------------
        -for loops
        -variants of for loops


 */


 fn main()
 {
 
     let mut some_vec=vec![2,3,4,5,6,7];
 
     //for i in 0..=5{
         //println!("the {}th value in vector is {}",i,some_vec[i]);
     //}
 
     for i in some_vec.iter_mut(){
         *i +=5; 
         println!("{}",i);
     }
     //changing owner ship error occur here so we use .iter fun 
     //println!("{:?}",some_vec);
     
 
 }