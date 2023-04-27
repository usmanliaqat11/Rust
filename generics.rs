   // -------------------------------------------
   // 			Generics
   //           	- Motivation (reducing code duplication)
   //           	- Generics in functions
   //           	- Generics in structure
   // -------------------------------------------

//As as we will be writing the code with lots and lots of duplication, in that case, the generics comes
//into play here and it allows us to write code by minimizing code duplication.


/*
fn squarei32(x:i32) -> i32{
    x*x
}

fn squaref32(x:f32) -> f32{
   x*x
}

fn main(){
   println!("The square of the number is {}",squarei32(5)); 
   println!("The square of the number is {}",squaref32(5.0)); 
   
}
*/
/*


//It basically tells the rest compiler that I am using a generic which is restricted to any type which
//implements the trade of multiplication and copy.Please note that we discussed in quite some detail the 
//difference between copy and move.The primitive types are copied and not moved, so this means that the 
//type T which is generic, but it does not mean that it can be anything. We are rather restricting it to 
//types that have the multiplication trait and the copy trait.

//fn square<T: std::ops::Mul<Output = T> + Copy + std::ops::Add<Output = T>> (x: T) -> T{           
    // another way of writting the first line 
    fn square<T> (x: T) -> T
    //the type of variable x to be that of T.
    where T: std::ops::Mul<Output = T> + Copy {
  
    x * x 
}


fn main(){
    println!("The square of the number is {}",square(5)); 
    println!("The square of the number is {}",square(5.5)); 
   
}

*/


/*
struct Point{
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point{x:5, y:5}; 
    
    let p2 = Point { x: 1.0, y: 4.0 };    
}
*/


/*
struct Point<T>{
    x: T,
    y: T,
}

fn main() {
    let p1 = Point{x:5, y:5}; 
    let p2 = Point { x: 1.0, y: 4.0 };      
   // let p3 = Point {x: 5, y: 5.0}; 
}
*/



struct Point<T, U>{
    x: T,
    y: U,
}


impl<T: std::fmt::Debug,U: std::fmt::Debug> Point<T, U>{  
    // Another way of writting the same is 
    /* 
    impl<T,U> Point<T, U>  
    where T: std::fmt::Debug, U: std::fmt::Debug {
    */      
    fn printing (&self) {
        println!("The value of the points are {:?}, {:?}", self.x, self.y);
    }

    
}
fn main() {
    let p1 = Point{x:5, y:5}; //in case of first point, the rest compiler considers both the fields as i32
    p1.printing(); 
    let p2 = Point { x: 1.0, y: 4.0}; // in case of second point point, the rest compiler considers both the fields f64
    let p3 = Point {x: 5, y: 5.0};  //in case of third point However, one of the field is considered to be either i32  and the other one is f 64.
    //All this is possible due to use of generics.

    //The point to note is that the type of the generic will be deduced by the compiler for each instance
    //of the struct separately.
}
 

