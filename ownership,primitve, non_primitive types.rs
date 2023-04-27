/* 


//owner ship is the new concept in rust which doesnt exist in programing languages. it is the most unique feature in rust which
//enable us memroy safety gurantee


/*

---------------------rust ownership-----------------------
//               -each value in rust has a variable that is called its owner-
//               -there can be only one owner at a time in rust-
//               -when the owner goes  out of scope, the value will be dropped



*/


fn main()
{

    let x=32.6;
    let y=x;

    println!("this is value of x {} this is the value of y {}",x,y);

    //program run without any error


    let s1:String=String::from("abc");
    let s2=s1;

    println!("s2 {}",s2);

    //now you it gives us error here.
    // here we learn the concept of move and copy 

    //--------------------key-point(1)---------------

    /*in case of integer rust will make a copy when we wite y=x it means there will seprate memory locations for two variables

    in case of strings rust ownership comes into play and in this case it will not make a copy.
    now in this case s2 is the new owner of string "abc" a
    in this copy not occur here but move occur here.


    //------------------------key-point(2)Answer arise here why not ownership move in int-------------

    rust has two types of variables primitive and non primitive these these are not data types. 
    //primitive type cannot be empty and fix size.
    //non primitive can grow and can be empty


    //with the primitve the rust will make another copy when it sees another assignment
    //however with the non-primitive rust do a move and ownership will be changed when it sees a assignment






    */

    //--------------key-point(3)---------------------
    //refrence do not change the ownership. but move change the owner ship

    //------------------this is refrence example

    let s1:String=String::from("abc");
    let s2:&String=&s1;

    println!("s1: {}, s2 {}",s1,s2);



/*  --------------------this is ownership example
    let s1:String=String::from("abc");
    let s2=s1;

    println!("s2 {}",s2);

*/ 

//---------------see the example of vector-------------------


// let vec1=vec![1,2,3,4,5];

// let vec2=vec1;
// println!("vec1: {:?}, vev: {:?}", vec1,vec2);


let vec1=vec![1,2,3,4,5];

let vec2=&vec1;
println!("vec1: {:?}, vev: {:?}", vec1,vec2);


//key point we want to make copy and do not change the ownership

let vec1=vec![1,2,3,4,5];

 let vec3=vec1.clone();

 println!("vec1: {:?}, vev: {:?}", vec1,vec3);

 //vec3 and vec1 are two diffrent owner and make copy 



 //end ------------of first two rules here-----------------------


 {
    let  my_name=String::from("usman");
 }

 println!("the value of my name is : {}", my_name);

 //value are out of scope here so thats why its gives us error;












}


*/ 