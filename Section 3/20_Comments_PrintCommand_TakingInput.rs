fn main()
{
    // comment start with '//'
    /*
        Comments can also be written like this in rust,
        this is a multiple line comment
     */

    //print macro
    print!("This is a print command"); // this will not insert a new line while printing

    /* Escape Sequences
        \n : Newline charater.
        \t : Tab space.
        \r : Carriage Return.
        \" : Double quote
        \\ Backward slash.
    */

    println!("\n will be printed after one empty line");
    println!("\t A tab space at the start");
    println!("This will be overwritten \r This text will only appear on screen")
    

    // Positional Arguments
    println!(
        "I am doing {2} from {1} years and i {0} it", "like",20,"programming"
    );

    println!("{language} is a system programming language which is cool to {activity} in.", activity="code" , language="Rust" );

    // Inputs standard Input out library is used

    let mut n:String =String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("failed to read input");
    // It's result will have two outputs either an okay(indicates operation is sucessful) or an error.

    let n:f64 = n.trim().parse().expect("Invalid Input"); // trimming step

    //Variable Conventions and Unused variables
    // name of variables is taken in snake type convention
    let _number_one:f64=45.0;
    let x:i32=40_000; //this convention can be used to represent very large numbers


    // Static Variables : Are like constants, Difference is that Constant are inline
    // Naming convention in big alphabets seprated by '_'
    static WELCOME: &STR="Welcome to Rust";
    const PI:f32 =3.14;
    // a and b will not occupy any memory they will refer to other one
    let a:f32=PI;
    let b:f32=PI;

    // statics will occupy memory for every instance
    let c: &str = WELCOME;
    let D: &str = WELCOME
}