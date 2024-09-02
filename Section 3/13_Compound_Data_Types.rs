fn main() {
    // &str and String
    let fixed_str: &str = "Fixed Length String"; // String slice, immutable
    let mut flexible_string: String = String::from("String will grow"); // Growable string, mutable
    flexible_string.push('s'); // Adding a single character to the string

    // Arrays
    let mut array_1: [i32; 5] = [4, 5, 6, 8, 9]; // An array of i32 elements with fixed size
    let num = array_1[3]; // Accessing the 4th element of the array

    println!("{:?}", array_1); // Printing the array
    let array_2: [i32; 10] = [0; 10]; // An array of 10 elements, all initialized to 0

    // Vectors: Can grow or shrink in size
    let mut vec_1: Vec<i32> = vec![4, 5, 6, 8, 9]; // A vector of i32 elements
    vec_1.push(10); // Adding an element to the vector

    // Tuples: Can contain values of different types
    let my_info: (&str, i32, &str, i32) = ("Salary", 40000, "Age", 40);
    let salary_value: i32 = my_info.1; // Accessing the second element of the tuple
    let (salary, salary_value, age, age_value) = my_info; // Destructuring the tuple

    // Unit Type Tuple
    let unit: () = (); // Unit type tuple, implicitly returned by functions with no return value
}
