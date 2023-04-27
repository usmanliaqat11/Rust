fn main()
{
    //break for stoping a loop

    let mut var=100;
    loop {
        var=var-1;

        if var%13==0
        {
            break;
        }
    }

    println
    !("the highest number lesser than a given number divisible by 13 is {}", var);


    let mut num=0;

    let mut count=0;

    loop{
        num=num+1;

        if num%5==0 && num%3==0{
            println!("the number which is divisible by both 3 and 5 is {}",num);
            count=count+1;

            if count==3
            {
                break;
            }

            else {
                continue;
            }
        }
        println!("{}",num)
    }
}


