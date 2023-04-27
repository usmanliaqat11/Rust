/*

-------------------------------------
        -loops with no condition
        -while loop


 */


 fn main()
 {
 
     //loop{
       //  println!("this is infinite loop");
     //}
 
     let my_number =5;
     let mut guess=false;
 
     println!("guess my number which is between 1 and 20");
 
 
     while guess!=true 
     {
 
         let mut number : String= String::new();
         std::io::stdin().read_line(&mut number).expect("failed to read input");
 
         let number: u8=number.trim().parse().expect("invalid input");
 
 
         if my_number==number{
             println!("you guessed the number correctly");
 
             guess=true;
         
         }
 
         else  {
             
             println!("please try again");
         }
 
 
 
     }
 
 
     println!("enter a number and i will tell you a number that is divisible 
     by both 2 and 5");
 
     let mut number1=String::new();
     std::io::stdin().read_line(&mut number1).expect("failed to input");
 
     let mut number1: u8=number1.trim().parse().expect("invalid input");
 
     let mut divisible_by_2_5=false;
 
     while divisible_by_2_5!=true {
         number1=number1+1;
 
         if number1%2==0 && number1%5==0
         {
             println!("the number after your number is divisible by both 2 and 5  is {}", number1);
             divisible_by_2_5=true;
         }
     }
 
 
 
 
 }