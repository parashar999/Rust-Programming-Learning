
/*
    -Borrowing Establish a refrence to some data
        -Just  lik pointers with some rules
        -Does not take Ownership
    -Motivation behind borrowing
        -Prevent unnecessary memory usage
        -Ownership not required
    -Rules
        -At any time, you can have either one mutable refrence or any number of immutable refrence
        -Refrences must always be valid.
    -Rules Solve out two problems
        -Data race
        -Dangling prefrences
*/

fn main()
{
    let mut vec_1: Vec<i32>=vec![4,5,6];
    let ref1: &mut Vec<i32>=&mut vec_1; // creating mutable refrence
    // An immutable reference allows you to borrow the data without changing the contents.
    // An immutable reference allows you to borrow the data and also change it.
    // No borrowing take ownership.
    
    /* 
        `this can't be done because it is voilating condition one. Code will not compile here, We will get an error 
        because within the scope of ref1 we should no other mutable reference to the data which is being violated by ref two`
        let ref2: &mut Vec<i32> = &mut vec_1;
        println!("ref1:{:?}, ref2:{:?}", ref1, ref2);
    */

    /*
        `Here code will compile. This is because the rust compiler counts the active period of reference, 
        also called the scope of reference and it is counted from a line in which it is introduced or 
        defined until the last line in which that reference is being used.Here Scope of ref1 is 
        limited to line no. 20 and scope of ref2 is limited to line no 37. There is no overlap, 
        meaning that at any given time we have only one mutable reference , hence no violation of rule` 
        
        let ref2: &mut Vec<i32> = &mut vec_1;
    */ 
    let ref1:&Vec<i32>=&vec_1; // Muttable Reference
    let ref2:&Vec<i32>=&vec_1;
    println!("ref1:{:?}, ref2:{:?}", ref1 , ref2); // this will work ,because it is not voilating any rule.

    /*
        `this can also not be done`
        let ref3:&mut Vec<i32> = &mut vec_1;
        println!("ref1:{:?}, ref2:{:?}, ref3:{:?}", ref1 , ref2, ref3);
    */
    
    /*
         -By following rule 1 rust avoid the problem of data race at compile time. 
         -A data race condition occurs when there are multiple references to the same data, with at least one
          updating the data, and there is no mechanism to synchronize access to the data.
         -By enforcing the borrowing rules, Rust either allows reading of data through multiple immutable references,
          or updating or writing to data through a single multable reference, This ensures that multiple parts of your 
          code can safely interact with the data withoug causing a race condition.
    */

    let vec_2: &Vec<i32> = {
        let vec_3: Vec<i32>= vec![1,2,3];
        &vec_3
    }; // This is a condition of dangling references because vec_3 is declared in scope, it will be cleaned after end of scope
    // This is a voilation of rule 2.

}