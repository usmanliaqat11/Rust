fn main()
{
    /*
    -----------------compund data types
    strings:-

    string is two types 
    1)&str
    2)String'

    &str some time refferd as sting slices
    string slices has fixed size and cannot be mutated means they are immutable and cannot changes its value.



    */
    // let some_string="fixed length";

     let mut grow_able_string: String=String::from ("this string will grow");
     println!("the string is : \"{} \" ",grow_able_string);

    grow_able_string.push('s'); 
    println!("the string is: \" {} \" ", grow_able_string);
    grow_able_string.push_str("which i will use");


    // grow_able_string.pop(); 
    // println!("the string is: \" {} \" ", grow_able_string);


    println!("Basic string funs, 
    is_empty(): {},
    length: {},
    bytes_size: {},
    contains : {}", grow_able_string.is_empty(),
    grow_able_string.len(), grow_able_string.capacity(),
    grow_able_string.contains("use"));

    grow_able_string.push_str(" ");
    
    //.trim fun removes the white spaces in string
    let str_len=grow_able_string.trim().len(); 
    println!("this is the trim string: {}",str_len);

//contains only tells about the string not char


//convert numeric variable to string

let number=6;
let num_str=number.to_string();
println!("is the number really a string: {} ",number.to_string()=="6");
println!("is the number : {} ",num_str);

let char_var='h';
let string_cah_var=char_var.to_string();

let my_name:String="nouman azam".to_string();

/*This is done because string literals are of type &str, not String, 
and &str does not support all the operations that a String does. 
By converting the string literal to a String, 
we can perform operations such as appending, changing, or removing characters from the string. */

// creating a empty string

let empty_strings:String=String::new();
println!("empty string length is {}",empty_strings.len());

let s_1:String="nouman".to_string();
let s_2:String="azam".to_string();
let s_3:String=format!("my first name is : {} and my second name is: {}",s_1,s_2);
println!("{}",s_3);


}