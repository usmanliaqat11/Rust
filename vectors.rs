fn main()
{
    //---------------vector------------------- 

    //it is similar like arrays but the key diffrence 
    //is it is re sizeable. which means it has no fix 
    //length. all elements in array are same type

    let mut num_vec=vec![1,2,3,4,5,6,7];
    println!("the values of vec{:?}",num_vec);

    num_vec[3]=5;

    num_vec.push(5);

    println!("the push values of vec{:?}",num_vec);
    num_vec.remove(1);
    println!("the remove values of vec{:?}",num_vec);

    // check the specific value available or not in vec
    println!("the num conatin vector or not is : {:?}",num_vec.contains(&10));

    let mut array_with_same_element=vec!["usman","liaqat"];
    println!("{:?}",array_with_same_element);
    array_with_same_element=vec!["cs"];
    println!("{:?}",array_with_same_element);


    let mut char_vec=vec!['a','b','c','d'];
    println!("{:?}",char_vec);




    






}