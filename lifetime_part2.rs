   // -------------------------------------------
   // 			Lifetimes  
   //           	- Dangling Reference 
   //           	- Undetermined Lifetimes
   //           	- Generic Lifetimes Parameters (GLP)
   //           	- GLP typically (not always) needed with outputs from functions that are references
   //           	- Issues with GLP
   //           	- GLP with multiple variable
   //           	- GLP and structures
   //           	- Reference to same variable
   // -------------------------------------------


//The very first point to note is that the generic lifetime parameters are only needed when we use 
//references as output from a function.

/*
fn main(){
    let s_1 = "Hello";
    let v;
    {
        let s_2 = String::from("World");       
        v = some_fn(s_1, s_2.as_str());    
    }                                    
    println!("{}", v);                   
    }
    
    fn some_fn<'a, 'b>(first_str: &'a str, second_str: &'b str) -> &'a str {
        first_str          
    }
    */
     

    
    
  /*  
    fn main(){
        let int1 = 5; 
        let int2 = 10; 
        let result = greater(&int1,&int2);
        
    }
    
    fn greater(i:&i32,j:&i32) -> i32 {
        if i> j {
            *i                   
        } else {
            *j          
        }
    }
*/



    
    
    /*
    fn main(){
        let int1 = 5; 
        let int2 = 10; 
        let result = greater(&int1,int2);
        
    }
    
    fn greater<'a>(i:&'a i32,j:i32) -> &'a i32 {   
        i
    }
 
    */


//--------------------The next point we want to focus is is on the issue with multiple lifetimes.--------


  /*  
    fn main(){
        let int1 = 5; 
        let int2 = 10; 
        let result = greater(&int1,&int2);
        
    }
    
    fn greater<'a,'b>(i:&'a i32,j:&'b i32) -> &'a i32 {  
        if i >j {
            i
        } else {
            j                          
        }
    } 

//the compiler anticipates that there may be situation and possibility for the variable J to be the 
//to be the returning reference.
//So therefore it sees in advance that the variable j does not match the lifetime of 
//the returning variable.  
    
*/

/*


//--------------------GLP with multiple variable---------------------------------------
fn main(){

    let int1 = 5; 
    {
        let int2 = 10;
        let result = greater(&int1,&int2);
        println!("The large value is {}", result);
    }
    
}

fn greater<'a,'b>(i:&'a i32,j:&'a i32) -> &'a i32 {  
    if i > j {
        i
    } else {
        j                          
    }
} 
*/
            
//------------------------------GLP and structures----------------------------------
 /*
    struct Person<'a>{
        name: &'a str,
        age: i32,
        }

        fn main() {
            
            
                let first_name = "Nouman";
                let mut nouman = Person {
                name:&first_name, 
                age: 40,
                };

                {
                    let last_name = String::from("Azam"); 
                    nouman.name =&last_name; 
                }

           println!("\n\n The name of the person is {} and his age is {} \n\n", nouman.name, nouman.age);   

        }


*/

//-----------------------------------Reference to same variable-----------------------

fn main(){
    let some_vec:Vec<i32> = vec![5,8,9,8,7,5,2] ; 
    let return_vec =  use_vec(&some_vec, &some_vec);  

}
//if both the parameters are smae it also gives error it doesnt know which one to be returning
 
fn use_vec<'a>(vec1: &'a [i32], vec2:&'a [i32]) -> &'a [i32]{    
    if 3>5 { vec1 

    }else {
        vec2

    }

}

//-------------------------------------imp note------------------------------------------------

//The special point to note is that the lifetime will come into play when we have 
//functions that returns references or when we have structures with a reference field.