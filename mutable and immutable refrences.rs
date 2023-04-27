fn main()
{
  /*
  refrence rules  

  -one mutable refrence in a scope
  -many immutable refrence 
  -mutable and immutable cannot coexist 
  -scope of a refrence 
  -data should not change when immutable refrence are in scope 

*/

//-------------------rule1------------- 
//it works it does not give us error
// let mut heap_num=vec![1,2,3,4];
// let ref1=&mut heap_num;
// println!("ref1 {:?}",ref1);


//it gives us error
// let mut heap_num=vec![1,2,3,4];
// let ref1=&mut heap_num;
// let ref2=&mut heap_num;
// println!("ref1 {:?} ref2{:?}",ref1,ref2);


//------------rule2------------------
// here we create multiple immutable refrence it allow us and its memory safe
let mut heap_num1=vec![1,2,3,4];
let ref2= &heap_num1;
let ref3= &heap_num1;
println!("ref2 {:?} ref3{:?}",ref2,ref3);


//----------rule3---------------------

let mut heap_num1=vec![1,2,3,4];
let ref2= &heap_num1;
let ref3= &heap_num1;
let ref4=&mut heap_num1; 
println!("ref2 {:?} ref3{:?}, ref4: {:?}",ref2,ref3,ref4);


//-------------rule4------------------
let mut heap_num1=vec![1,2,3,4];
let ref2= &heap_num1;//start scope here ref2 
let ref3= &heap_num1;
//start scope here ref3
println!("ref2 {:?} ref3{:?}",ref2,ref3); //end scope here both of ref2 and ref3

let ref4=&mut heap_num1; 


//-------------rule5------------------

let mut heap_num1=vec![1,2,3,4];
let ref2= &heap_num1;
let ref3= &heap_num1;

//heap_num1.push(58); here error
println!("ref2 {:?} ref3{:?}",ref2,ref3);

heap_num1.push(58);

}