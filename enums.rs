   // -------------------------------------------
   // 			Enums
   //           	- General Syntax
   //           	- Enums with attached data
   // 			- Using enums to create vector with different types of data
   // -------------------------------------------

//what is the diffrence of enum and struct
//The key difference, however, is that in case of structure, we define the type of each and every field.
//However, we do not need to define the types in case of enum 


// enum Conveyance {
    // Car , 
    // Train ,
    // Air, 
// }
// 
// impl Conveyance {
   //this fun has two input The first one will be a reference to the variant for which we are calling it.
    //And Miles, a data participant, has traveled.
// 
//  
    // fn travel_allowance(&self, miles:i32) -> f32 {
        // let allowance  = match self {
            // Conveyance::Car => miles as f32 * 14.0 * 2.0,  
            // Conveyance::Train => miles as f32 * 18.0 * 2.0,
            // Conveyance::Air => miles as f32 * 30.0 *2.0,
        // }; 
        // allowance
    // }
// }
// 
// 
// fn main() {
//  
    // let participant_1 = Conveyance::Car; 
// 
   // println!("the value of the option is {}", participant_1 as i32);
    // let participant_2 = Conveyance::Air; 
    // let participant_3 = Conveyance::Train; 
// 
// 
// 
    // println!("The participant 1 has a travel allowance of {}", participant_1.travel_allowance(60)); 
    // println!("The participant 2 has a travel allowance of {}", participant_2.travel_allowance(120)); 
    // println!("The participant 3 has a travel allowance of {}", participant_3.travel_allowance(60)); 
// }
// 
// 





/*
enum Conveyance {
    Car(i32), 
    Train(i32),
    Air(i32), 
}

impl Conveyance {
    fn travel_allowance(&self) -> f32 {
        let allowance  = match self {
            Conveyance::Car(miles) => *miles as f32 * 14.0 * 2.0,  
            Conveyance::Train(miles) => *miles as f32 * 18.0 * 2.0,
            Conveyance::Air(miles)=> *miles as f32 * 30.0 *2.0,
        }; 
        allowance
    }
}


fn main() {

   
    let participant_1 = Conveyance::Car(60); 
    let participant_2 = Conveyance::Air(120); 
    let participant_3 = Conveyance::Train(60); 
    
    

   println!("The participant 1 has a travel allowance of {}", participant_1.travel_allowance()); 
   println!("The participant 2 has a travel allowance of {}", participant_2.travel_allowance()); 
   println!("The participant 3 has a travel allowance of {}", participant_3.travel_allowance()); 
   }
   */



//to make sure that the enum uses the standard format debug which is required for printing.
//This will ensure that the enum uses the debug trait which is used for printing purposes.

#[derive(Debug)]
enum Value{
    Integer(i32), 
    Float(f32),
}
fn main() 
{

    let some_val = vec![Value::Integer(12), Value::Float(15.5)]; 
    println!("\n\nThe value of the integer is {:?}, {:?}", some_val[0], some_val[1]); 

    for i in some_val.iter(){ 
    match i {
     //The num will be assuming values that are attached with the respective variance.
        Value::Integer(num) => println!("The value is an integer with a value of {} ", num), 
        Value::Float(num) => println!("The value is an float with a value of {}", num),  
        }
    
    }
 //It must be clear that we have not violated the key requirement for a vector, which is that it only
 //contains values of a single type.
 //In this case, the vector is containing values of type value, which is our enum.
 //More specifically, considering enum as a type enables us to store various types of data inside the
 //vector without breaking its necessary requirement.   

}

