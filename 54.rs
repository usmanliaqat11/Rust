    // -------------------------------------------
    // 			Smart Pointers
    //           	- Custom Defined Smart pointers
    //            	- Deref Coercion  
    // -------------------------------------------



/* 

//bfore code 


 struct MySmartPointer{value: i32}    // add it at the end 
 
 
     impl MySmartPointer {
         fn new(x:i32)-> MySmartPointer<T> {
            MySmartPointer{value: x}
     
         }
     }

      
      use std::ops::Deref; 

     impl Deref for MySmartPointer {
type Target = i32;
fn deref(&self) -> &i32 {
  &self.value 
  }
}

 
 impl Drop for MySmartPointer{
     fn drop(&mut self){
        println!("dropping MySmartPointer object from memory {:?}", self.value);
     }
 }







*/

//updated code

//you see by doing little changes we gernalize our data that associated with smart pointer
struct MySmartPointer<T: std::fmt::Debug>{value: T, 
    name: String}    // add it at the end 


    impl<T: std::fmt::Debug> MySmartPointer<T> {
        fn new(x:T)-> MySmartPointer<T> {
           MySmartPointer{value: x, name:String::from("Hello")}
    
        }
    }
    
    use std::ops::Deref; 
    impl<T: std::fmt::Debug> Deref for MySmartPointer<T> {
    type Target = T;
    fn deref(&self) -> &T {
      &self.value 
      }
    }
    
    
    
    impl<T: std::fmt::Debug> Drop for MySmartPointer<T>{
        fn drop(&mut self){
           println!("dropping MySmartPointer object from memory {:?}", self.value); //if we dont use std::fmt::Debug 
            //it will give us error here self.value to jha trait use ho ge uskay sath ye use std::fmt::Debug krna ho ga
        }
    }
    

    fn my_fn(str: &str) 

    {
       println!("The string recieved from teh main is \"{}\"", str); 
    }
     
 
    #[derive(Debug)]
   struct Person{
   name: String, 
   
}


 fn main() { 
   let sptr_p1 = MySmartPointer::new("Nouman Azam"); 
   my_fn(&sptr_p1); //&MySmartPointer->&String->&str
   //second read this //deref call goes like this first deref call mysmartpointer and return &String another 
   //deref is call on string it retun &str string slice

   //first read //main say jo call bejhi hai wo check kry ga ye deref trait hai ya nhi if yes 
   //f yes, it will try to call the deed of trade on it. so the deref method call on which return the refrence
   //to the inner value

   let some_vec = MySmartPointer::new(vec![1,2,3]);
   
   for z in &*some_vec { 
       println!("The value is {}", z);

//Please note that if I remove the & a before the start, then the compiler will complain and it says
//that this will move the values from the smart pointer. Since the Dref is called using the START operator and the 
//last compiler is smart enough to know that whenever a deref is called so a reference would be returned.

//simple is that if i remove & sign from here so it will move smartpointer to i32 so it will give us error
//if i use & sign so it will give us refrences of vectors values
   }   

 }

