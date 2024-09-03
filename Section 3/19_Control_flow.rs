fn main()
{
    loop // simple loop conditional
    {
        println!("Simple loop");
        break; // if we have nested loops , the break will only exit out from the inner loop
    }

    'outer: loop
    {
        // code here
        break 'outer; // to explicitly call the break statement in nested loops we label the loop the label starts with '
    }

    // loop is also an expression, therefor

    let a:i32=loop{
        break 5; // 5 will be returned when the loop breaks
    };

    // for loop

    let vec= Vec<i32> =vec![45,30,85,90,41,39];

    for i:i32 in vec
    {
        println!("{i}");
    }

    // While loop
    let mut num:i32=0;
    while num<10
    {
        num=num+1;
    }

}