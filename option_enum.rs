   // -------------------------------------------
   // 			Option enum 
   //           	- Basic Syntax and usage
   // -------------------------------------------



//So the first thing to note about the option enum is that it has two variants.
//The first variant is called None, which is used to indicate failure or lack of value.
//And the second variant is called Sum, which is basically a tuple struct that wraps a value with type t  
//Where is a generic type Due to its importance The option enum are included in the programs by default.

   /* The General Syntax

 //t uses the generic type T which can be any type, and there are two variants none and some   
   enum Option<T> {
    None,
    Some(T),
    }
   */ 


   // Example 1
/* 
fn main (){

    let mut disease: Option<String>= None; 
    disease = Some(String::from("Diabetes")); 
    
    match disease {
        Some(disease_name) => println!("You have the disease of {}",disease_name),
        None => println!("You do not have any disease"),
    }
}
*/

// Example 2. Options can be of any type 

/*
struct Person {
    name: String, 
    age:i32,
}
fn main() {

    let s1: Option<&str> = Some("Some String");  
    
    println!("\n The value of s1 is {:?}\n The value of s1 itself is {:?} \n\n", s1, s1.unwrap());

    
    let f1: Option<f64> = Some(10.54);


    
    let mut f2 = 16.5; 
    f2 = f2+f1.unwrap(); 
    println!("the value of the sum is {}", f2);
    

    let v1: Option<Vec<i32>> = Some(vec![0, 1, 2, 3]); 
    

    let p1 = Person {
        name: String::from("Nouman"),
        age: 32,   // even this can be set to optional 
    }; 

    let someone: Option<Person> = Some(p1); 

    
}
*/


//As a final example, we will see how to pass a variable of option type as a function parameter.

fn main() {
    let number = Some(6);

    if square(number) != None {
        println!("\n\n The result of the square is {:?} \n\n", square(Some(6)).unwrap()); 

    } else {println!("\n\n We do not have any value \n\n"); }
    square(None); 

}

fn square(num: Option<i32>) -> Option<i32> { 
    match num {
        Some(number) => Some(number * number),  // Since the return is an option therefore we need to wrap the answer in the some  
        None => None,
    }
}