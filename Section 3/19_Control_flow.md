# Control Flow in Rust

### 1. **`loop` Statement**
- **Infinite Loop**: The `loop` construct creates an infinite loop that runs until it's explicitly broken using a `break` statement.
  ```rust
  loop {
      println!("This will print indefinitely");
      break; // Exits the loop
  }
  ```

- **Labeled Loops**: When dealing with nested loops, you can label loops to specify which loop to break out of. The label is followed by a colon (`:`).
  ```rust
  'outer: loop {
      loop {
          break 'outer; // Exits the outer loop
      }
  }
  ```

- **Returning Values**: The `loop` can return a value when it exits. This makes `loop` an expression.
  ```rust
  let result = loop {
      break 42; // 'result' will be assigned the value 42
  };
  println!("Result is: {}", result);
  ```

### 2. **`while` Loop**
- **Conditional Looping**: The `while` loop continues running as long as the specified condition is `true`.
  ```rust
  let mut count = 0;
  while count < 5 {
      println!("Count is: {}", count);
      count += 1;
  }
  ```
  - The loop will stop running once `count` reaches 5.

### 3. **`for` Loop**
- **Iterating Over a Collection**: The `for` loop is typically used to iterate over elements in a collection (e.g., array, vector, range).
  ```rust
  let numbers = vec![1, 2, 3, 4, 5];
  for num in numbers {
      println!("Number: {}", num);
  }
  ```

- **Range-based Looping**: You can loop over a range of values using `..` (exclusive end) or `..=` (inclusive end).
  ```rust
  for i in 0..5 { // i goes from 0 to 4
      println!("i: {}", i);
  }

  for i in 0..=5 { // i goes from 0 to 5
      println!("i: {}", i);
  }
  ```

### 4. **`continue` and `break`**
- **`break`**: Exits the loop immediately.
  ```rust
  for i in 0..10 {
      if i == 5 {
          break; // Loop exits when i equals 5
      }
      println!("i: {}", i);
  }
  ```

- **`continue`**: Skips the rest of the current loop iteration and moves to the next iteration.
  ```rust
  for i in 0..10 {
      if i % 2 == 0 {
          continue; // Skips even numbers
      }
      println!("i: {}", i); // Only prints odd numbers
  }
  ```
