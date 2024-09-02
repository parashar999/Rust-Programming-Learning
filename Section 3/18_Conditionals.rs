fn main()
{
    let num=40;
    // if else only take true or false values it will not accept any other value in condition
    if num<50 { 
        println!("The number is less than 50");
    }
    else{
        println!("The number is greater than or equal to 50");
    }

    

    // if, else if ladder
    // if-else and if-else-if ladder are expressions so they returns value which can be stored, ';' used while assignment
    // all types of value returned should be of same type
    if num>50
    {
        // code here
    } 
    else if num>70
    {   
        // code here
    }
    else if num>90
    {
        // code here
    }

    // Matching Patterns

    let mut grade:char='N';
    let marks:i32=95;
    match marks // match is also an expression
    {
        90..=100 => grade='A', // right pattern to match , left code to execute, if there are multiple lines in code they are written within { }
        80..=89 => grade='B', // Value from 80 to 89
        70..=79=> grade='C',
        _=>grade='F', // default arm , _ matches everythig, this should be written for saftey
    }
}