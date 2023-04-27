   // -------------------------------------------
   // 			Closures   
   //           	- A quick recap
   //           	- Borrow by immutable reference
   //           	- Borrow by a mutable reference
   //           	- Moving of a value into a closure
   // -------------------------------------------



/*

//We will first look at the case when the closure uses an immutable reference to a variable inside the
//code
fn main(){

let some_closure_1 = |x: u32| -> u32 { x + 1 }; 
let some_closure_2 = |x| { x + 1 };    
let some_closure_3 = |x|  x + 1 ; 
} 
 */



//We will first look at the case when the closure uses an immutable reference to a variable inside the
//code

//Let us now understand what happened in this case.
//The first thing to note is that we do not have mentioned the input to this closure.
//However, since the closure captures its environment, so all the variables which are currently in scope
//will be available inside the closure also.
//since the value inside the closure is being used as an immutable refrence 
//VEC underscore one is available for us as it is being used by reference. 

/*
fn main(){
let mut vec_1 = vec![1, 2,3];
let some_closure = || {
    // vec_1 is being used by reference.  
    println!("Vec 1 : {:?}", vec_1);     
};

println!("Vec_1: {:?}",vec_1);   
vec_1[1] = 15;//the rest compiler will rightfully complain.
//This is because the vector is being borrowed as immutable and 
// we cannot mutate or change its value until the immutable reference goes out of scope.

some_closure();    
                   
vec_1[1] = 15; //here out of scope so it will create no issue


}
*/ 


//Now let us see what happens when a closure uses a mutable reference to some variable.
//I will change the statement inside the closure, so I will delete the statement and will push a value  


fn main(){
    let mut vec_1 = vec![4,5,6];
    //if i not write the mut so it will give error
    //because one immutable refrrence in scope so value not update ownership rule 5
    let  mut some_closure = || {
        
        vec_1.push(35); 
       
    };
    
    println!("vec_2 {:?}", vec_1);  //It is not allowed because it is being borrowed as mutable, since accessing the value is not allowed
   // vec_1[1] = 15; //this error is due to one mutable refrence in a scope                  
    some_closure();    
                        
    
    // vec_1[2] = 15; 
    
    }
    
/* 
   
fn main(){
    let mut vec_1  = vec![1,2,3];
    let some_closure = || {
        
        let vec_2 = vec_1;
        //println!("vec 2 = {:?} ", vec_2); vec_2 scope limited to the body
    };
    

    some_closure();     
    println!("vec 1 = {:?} ", vec_1);   //vec_1 is is no more in scope and has lost the ownership. to vec_2 in closure
    println!("vec 2 = {:?} ", vec_2);                                       
             
    
}

*/
    