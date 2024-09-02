fn main()
{
    my_fn("Hello Friend", 32); // calling function and sending value to funciton
    let str:&str="Function call withv variable";
    my_fn(str, 32);
    let x=multi(3,5); // value from function can be stored in a variable

    let result:(i32, i32, i32) = basic_math(10,15,20); // Storing returning value into tuple

    let (multiplication: i32, addition:i32, subtraction:i32) =basic_math(10,15,20); // Destructuring and storing into variables.


    {
        // code block
        // they can also return a value, last expression will be returning value
    }

    // format!("") marco is used for string formatting in rust
    // assigning the returning value of a code block to an variable
    let full name: i32 ={
        64 // last expression will be returning value
    }; // as whole code block is an assignment statement so we add a ';' at the end

    // Difference between functions and code blocks
    // 1. Code blocks are not designed for reuse while functions are
    // 2. Code block do not have explicit parameters all variables in scope are visible to it
}

fn my_fn(s:&str , x:i32) // function decleration and accepting parameters in function
{
    println!("{s}");
    println!("{}, x ");
}

fn multi(num1:i32 , num2: i32) -> i32
{

    return num1*num2 // To return early we return with return keyword

    // statements will not return any value like print statement
    num1*num2 // final expresion in function will return value, ';' should not be used while returning value

    // There can not be multiple returning expressions in a function
    // There should be only one returning experssion and that should be last expression in the function
}

fn basic_math(num1:i32 , num2:i32 ) ->(i32,i32,i32) // functions can return multiple values
{
    (num1*num2 , num1-num2, num1+num2)
}