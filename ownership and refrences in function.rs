//how to ownership change in functions


fn stack_function(mut var:i32)
{ 
    var=56;
    println!("var: {}",var)

}  

// fn heap_vector(var1: &Vec<i32>)

// {
//    println!("var: {:?}", var1);
// }



 fn heap_vector(var1: &mut Vec<i32>)

 {
    println!("this is the fun var: {:?}", var1);
 }


fn main()
{
    let stack_num=32;
    let mut heap_vec=vec![1,2,3,4,5];

    stack_function(stack_num);
    println!("this is inside the fun {}",stack_num);
   


    //-------------------key-point-----------------------
   //inside the main fun stack_num is immutable and when we pass the parameter stack_fun so that yime its chage to mutable 

   //value dont move on the stack from main fun. but it create the copy of that and pass to the parameter
  


    //-------------------key-point(2)-----------------------
    //owner ship change here and it gives us error for avoiding this error we give refrence to the heap_vec fun
  //heap_vector(heap_vec);
  //println!("thsi is the haep_vec: {:?}",heap_vec);



 //-------------------key-point(3)-----------------------

//in this case the owner ship will remain same 
  heap_vector(&mut heap_vec);
  println!("thsi is the in the main haep_vec: {:?}",heap_vec);


 //-------------------key-point(4)-----------------------
//if we want to change the ownership then we to use the below syntax


let large_data=String::from("my name is usman");
let large_data1=String::from("i am studemt ");

let huge_data=vec![&large_data,&large_data1];



}
