
/*
    Each value has a variable that's its "owner"
    A value can have only one owner at a time.
    If the owner goes out of scope, the value is cleaned up.
*/

fn main()
{
    let s1: String =String::from("world");
    let s2: String = s1; // Here Ownership of s1 goes to s2
    print!("s1 is : {s1}"); // this will give a compile time error, it is developed like this way to stop memory leak or handling dangling pointers.

    let s3: String = s1.clone(); // This will clone data in the heap and declare new s2 in stack pointing to new clone in heap
    // In rust terminology of copy is used only when the stack data is being copied.
    // In rust primitive data types are fully stored on stack therefore now such problems with them.
    /* 
    
        Non-Volatile Memory
        -Hard Drvies/SSD
        -Slow but abundant
        -Persist data

        volatile Memory
        -RAM/Cache
        -Fast and Scare
        -Used During Program Execution


        Three Different regions of Memory During Program execution
        1. STATIC : -Used to store programs, binary instructions, and static variables.
                    -This region is populated with relevant program data when your program starts up and destroyed when you program ends.
                    -The clean up of values from this part is automatic.
                    -The stack deals with data which has a fixed size at compile time.
                    -Since the size is known , the values are stored in order using LIFO strategy.
                    -The operations associated with this are simple and fast and do not require any special computations.
        2. HEAP :   -Deals with data which is unknown at compile time
                    -Data is stored all over the place where a suitable fit for the data can be found in memory.
                    -The management of this part is therefore typically slow and requires a lot of memory management during execution. 
                    -Stack and Heap can both grow in size while execution

    */ 
}