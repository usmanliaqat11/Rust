fn main()
{
    /*------compund data types 
    ---------tupels------------
    --------Arrays------------
     */

     //defining tuples

     let my_information=("salary",40_000);
     println!("{} is equal to {}",my_information.0,my_information.1);
     println!("another way of printing the value {:?}",my_information);

    //another way declaring tuples this is de structuring

     let(salary,salary_value)=my_information;

     let salary=my_information;
     let salary_value=my_information.1;

    let nested_tuple=(4,5.0,(3,0),"hello");
    let element=nested_tuple.3;
    // this is not possible in this way it will give us error let element=nested_tuple.2;
    println!("this is the nested tuple: {} ",element);

    //empty tuples
    let empty_tuples=();
    println!("empty tuples {:?}: ",empty_tuples);


    // --------------------Arrays-----------------------

    let mut_array=[1,2,3,4,5];
    println!("{}",mut_array[0]);
    println!("this is Array: {:?}",mut_array);
    

    //10 zeros of array
    let array_with_the_same_element=[0;10];

    let mut string_array_1=["applr","shake","grapes"];
    //for updating the value of ist index
    string_array_1[0]="usman liaqat";
    println!("the vales of strings of array {:?}", string_array_1);

    let char_array=['a','b','c'];


    //--------------array slices------------------------
    // slices are used to refer some subset of elemnts in the array
    //they make the copy of array but rather refrences to the array

    //we cannot used them to update the value of array

    let mut number_array_1=[1,2,3,4,5,"usman"];
    let subset_array=&number_array_1[0..=3];

    println!("the array is occupying {} bytes",std::mem::size_of_val(&number_array_1));

    //-------------------get fun(enum fun)---------------
    //if the particular value exit in the index it returns sum or not exist it return non

    let check_index=number_array_1.get(100);
    println!("{:?}", check_index);







}