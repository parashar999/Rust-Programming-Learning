fn main() {
    // Type Inference
    let x = 10; // i32 inferred
    let y = 32.0; // f64 inferred

    // Explicit Typing
    let x: i32 = 10; // 32-bit integer
    let k: f32 = 10.0; // 32-bit floating-point number

    // Compile-time Error Examples
    // let y: f64 = 32; // Error: mismatched types
    // let z: i16 = 10.0; // Error: mismatched types

    // Mutability
    // x = 20; // Error: cannot assign to immutable variable
    let mut a: i32 = 15; // mutable variable

    // Scope and Lifetimes
    {
        let z: i32 = 50; // z is scoped to this block
    }
    // let s = z; // Error: z is out of scope

    // Shadowing
    let t: i32 = 10;
    let t: i32 = t + 10; // shadowing, creates a new variable

    let v: i32 = 30;
    {
        let v: i32 = 40; // shadows outer v
        println!("{v}"); // outputs 40
    }
    println!("{v}"); // outputs 30

    // Constants
    const MAX_VALUE: u32 = 100; // constants are immutable and cannot be shadowed

    // Type Shadowing Example
    let s = "string";
    let s = s.len(); // s now shadows the previous s, changing its type from &str to usize
}
