

// -------------------------------------------
// 		Link List (Part 1) 
// -------------------------------------------

#[derive(Debug)]
struct linklist{
    //head:node
   head: pointer, 
}
#[derive(Debug)]

//struct is liye use kr rhy hai kyu enum at at a time aik value store krta hai 
//ya value store kry ga ya pointer..but we need both here..so thats why we use struct
struct Node{
    element: i32, 
    next: pointer,     
  }

type pointer = Option<Box<Node>>; 

fn main(){
   /* 
   
   let list = Node{element: 1, next: None}; //list create using one element

   let list = Node{element: 1, next: Some(Box::new(Node { //list to bn gye hai mgr head nhi hai
      element: 2, next: Some(Box::new(Node {
         element:3, next: None
      }))
   }))};

   let list = linklist {head: Node{element: 1, next: None}};
   
   let list = linklist {head: Node { element: 1, next: Some(Box::new(Node {
      element: 2, next: Some(Box::new(Node {
         element: 3, next: None
      }))
   })) }}; //However, it still does not have an empty head or in other words, an empty list.
           //This is because the head is of type node, which has two fields that we need to mention.
           //Although one of the fields can be none, but for the other one, we need to specify some value.

           //What if we make the head as an optional box node?
 */ 

 let list = linklist{head: None};

 let list = linklist{head: Some(Box::new(Node {element: 100, next: (
   Some(Box::new(Node {
      element: 200, next: None
   }))
 )}))}; 

 //println!("{:?}", list.head.unwrap().element);  
 //head is an optional box pointer So when we unwrap, we have the full head and then we will assess the element part of the head, which will give us the value associated with the head.


 //println!("{:?}", list.head.unwrap().next.unwrap().element);  

 println!("{:?}", list.head);



}