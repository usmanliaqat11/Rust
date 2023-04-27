 

fn main()
{
  println!("Enter a number:");

  let mut some_num = String::new();
  std::io::stdin().read_line(&mut some_num).expect("Failed to read input");

  let some_num:i32=some_num.trim().parse().expect("invalid input"); 

  if some_num!=0{
    if some_num%2==0
    {
      println!("the number is even");
    }

    else 
    {
      println!("number is odd");
    }

  }


let marks=70;
let grade=if marks>50
{
  'A';

}


else if marks>55
{
  'B';
  
}


else if marks>60
{
  'C';
  
}

else
{
  'F';
};


println!("the obtain grade is {}", grade);

}