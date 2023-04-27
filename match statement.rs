/*

-------------------match statement--------------------

 */


 fn main()
 {
 
     /*
     match value 
     {
 
       possible_value(s)=>{statement to execute},
       possible_value(s)=>{statement to execute},
       possible_value(s)=>{statement to execute},
 
       _ =>{default execution of some statement}
     
     
     }
 
      
     */
 
     let some_number=100;
     match some_number{
 
         1 =>println!("the number is 1"),
         2 | 3 =>println!("the number is 2 or 3"),
         4..=100 => println!("the number is between 4 and 100"),
 
         _=>println!("the number is greater than 100"),
 
         // in this case comiler gives us error although it will execute
         //first arm say 2 execute ho jaye ga mgr 2 arm say excute nhi ho ga
 
         //  1 |2 =>println!("the number is 1"),
        //   2 | 3 =>println!("the number is 2 or 3"),
 
 
        //defaut arm related 
        // if we want to execute all the arm then taht case is true or false so we dont need 
        // for default arm
         
 
     }
 
 
     let marks=50;
 
     let mut grade='N';
 
     match marks{
      90..=100 =>grade='A',
      80..=89 =>grade='B',
      70..=79 =>grade='A',
 
      _=> grade='F',
 
 
     }
 
     println!("the grade achive is {}", grade);
 
 
     //if let with match statement
 
     /* 
 
    let variavle=  match value{
   possible_value(s)=>{statement to execute},
   possible_value(s)=>{statement to execute},
   possible_value(s)=>{statement to execute},
   _ =>{default execution of some statement}
     
     
 }
     
     */
 
 
     let marks=98;
     let mut grade=match marks{
      90..=100 =>'A',
      80..=89 =>'B',
      70..=79 =>'A',
      _=> 'F',
     };
     println!("the grade achive is {}", grade);
   
 }
 
 