### 1. **String Types**:
   - **`&str` (String Slice)**: 
     - Immutable, fixed-length string stored somewhere in memory (often in the binary).
     - Example:
       ```rust
       let fixed_str: &str = "Fixed Length String";
       ```

   - **`String`**:
     - A growable, heap-allocated data structure. It is mutable and can change its size at runtime.
     - Example:
       ```rust
       let mut flexible_string: String = String::from("String will grow");
       flexible_string.push('s'); // Adds a single character to the string
       ```

### 2. **Arrays**:
   - Arrays are collections of elements of the same type with a fixed size. The size of the array is part of its type.
   - Example:
     ```rust
     let mut array_1: [i32; 5] = [4, 5, 6, 8, 9]; // An array of 5 i32 elements
     let num = array_1[3]; // Accessing the 4th element of the array (indexing starts from 0)

     println!("{:?}", array_1); // Printing the array
     let array_2: [i32; 10] = [0; 10]; // Declaring an array with 10 elements, all initialized to 0
     ```

### 3. **Vectors**:
   - Vectors are similar to arrays but can grow or shrink in size. They are heap-allocated and can be resized dynamically.
   - Example:
     ```rust
     let vec_1: Vec<i32> = vec![4, 5, 6, 8, 9]; // A vector of i32 elements
     vec_1.push(10); // Adds an element to the end of the vector
     ```

### 4. **Tuples**:
   - Tuples can contain elements of different types. They have a fixed size and can hold a mix of types.
   - Example:
     ```rust
     let my_info: (&str, i32, &str, i32) = ("Salary", 40000, "Age", 40);
     let salary_value: i32 = my_info.1; // Accessing the second element of the tuple

     let (salary, salary_value, age, age_value) = my_info; // Destructuring the tuple into individual variables
     ```

   - **Unit Type**:
     - The unit type `()` is a special tuple with zero elements. It represents the absence of a value and is implicitly returned by functions that do not return any other value.
     - Example:
       ```rust
       let unit: () = (); // The unit type
       ```

