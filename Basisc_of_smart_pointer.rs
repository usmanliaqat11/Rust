// -------------------------------------------
// 		Smart Pointers
//          	- Box Pointers 
// -------------------------------------------

//---------------------what is pointer------------------------ 

//the concept of having a variable that contains an address of some memory 
//location where the address essentially stores some data.
//they act as not only ponter but they have more capabilites of smart pointers


//in rust by default allocate memory on stack   
//box pointer jo refrence data hai usko heap pr store krwata hai 


fn main() {
    let single_value = Box::new(0.625); 
    //variable single_value is stored on stack but value o.625 is on haep due to smart pointer
    let x = 0.625; //this value is on the stack              
    println!("Are the values being equal {}", x == *single_value);   // deref is needed when box contains a single value
            
           
    let mut stack_var = 4; 
    let stack_ref = &stack_var; //both variables are on stack.. refrences are always on stack 
                                        //because they are fix in size 
    let heap_var = Box::new(stack_var);        // what happens when we write stack_var inside the (stack_ref)
    //stack_var itself not store on heap.. copy of stack_var is stored on heap and heap_var point that 
    //memory location 

    stack_var =  5; 
    println!("The value of stack_var = {} and heap_var = {}", stack_var, heap_var);
    

    let point = Box::new((100, 125)); 
    println!("{} {}", 100 == point.0, point.1);   
    
    let x = point; //     
}