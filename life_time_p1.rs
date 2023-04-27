   // -------------------------------------------
   // 			Lifetimes  
   //           	- Dangling Reference 
   //           	- Undetermined Lifetimes
   // -------------------------------------------

//The lifetimes explains the scope for which reference is being valid.
//Rust claims to be memory safe, and we have talked about that to some extent 
//under the topic of ownership.
//The concept of lifetimes in rust is also part of memory safety feature.


//---------------------------question----------------------------
//We will address the question of why we need lifetimes.
//We will need to understand what are dangling references when the 
//references are being used to point


//---------------------------question----------------------------
//The dangling reference is a reference which points to a resource whose owner does not exist.
//In other words, when a program tries to access an invalid reference, it is known as a dangling 
//reference.

//Dangling references are typically encountered when we are dealing with functions.

//    use std::vec;

/*
fn main(){
    let i:&i32;                 
    {                           
                            
        let j = 5;              
        i = &j;                 
    }                           
    println!("The value of i = {}", i);   
}                               
*/



fn main() {

    let some_int = 10;  
    let additional_int = some_fn(some_int);  
    println!("{}", additional_int); 

}
//This function will accept an integer input and will return a reference to an integer.
fn some_fn(i: i32) ->  &i32{    
    &i                            
}



/*
fn main(){
    let int1 = 5; 
    let int2 = 10; 
    let result = greater(&int1,&int2);
    
}

fn greater(i:&i32,j:&i32) -> &i32 {
    if i> j {
        i                   
    } else {j          
    }

}
*/ 




fn main(){
    let s_1 = "Hello";
    
    
    let v;
    {
        let s_2 = String::from("World");       
        v = some_fn(s_1, s_2.as_str());    
    }                                       
    println!("\n\n{} \n\n", v);             
    }
    
    fn some_fn(first_str: &str, second_str: &str) -> &str {
        first_str           
                            
    }
    