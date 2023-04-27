fn main()
{
  //simple if ->general synatx

let some_num=30;

if some_num<50
{
  println!("the number is less than 50")
}


println!("this line also execute irresprctive of if statement");

let marks=65;
if marks>=60 && marks<=70
{
   println!("the grade is statifactory");
}

let flag_1=true;
let flag_2=false;

if flag_1==true || flag_2==true
{
  println!("one of the condition is true");
}

let flag_1=true;
let flag_2=false;
let number=60;

if (flag_1==true && flag_2==false) || number<50
{
  println!("this part will be execute");
}

//if else examples

let marks=30;

if marks>50
{
  println!("you have passed")
}

else {
  println!("you are failed")
}



//if else ladder

let marks=70;
let mut grade='N';

if marks>50
{
  grade='A';

}


if marks>55
{
  grade='B';
  
}


if marks>60
{
  grade='C';
  
}

println!("the obtain grade is {}", grade);

}