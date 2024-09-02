
---

### **Conditionals in Rust**

1. **Basic `if`-`else` Statement**

    ```rust
    fn main() {
        let num = 40;

        if num < 50 { 
            println!("The number is less than 50");
        } else {
            println!("The number is greater than or equal to 50");
        }
    }
    ```

    - **Description**: The `if`-`else` statement evaluates a boolean condition. If the condition is `true`, the block of code associated with `if` is executed; otherwise, the block of code associated with `else` is executed.
    - **Notes**:
        - The condition in an `if` statement must be of type `bool`.
        - Rust's `if`-`else` is an expression, meaning it can return a value, but in this simple form, it's used for control flow.

2. **`if`-`else if` Ladder**

    ```rust
    fn main() {
        let num = 40;

        if num > 50 {
            // code here
        } else if num > 70 {   
            // code here
        } else if num > 90 {
            // code here
        }
    }
    ```

    - **Description**: An `if`-`else if` ladder allows for multiple conditions to be checked in sequence. The first condition that evaluates to `true` executes its corresponding block, and the rest are skipped.
    - **Notes**:
        - Each branch of the ladder must be mutually exclusive if intended to be.
        - Since `if`-`else if` is also an expression, you can assign its result to a variable if the branches return the same type of value.

3. **Pattern Matching with `match`**

    ```rust
    fn main() {
        let mut grade: char = 'N';
        let marks: i32 = 95;

        match marks {
            90..=100 => grade = 'A', // Matches marks from 90 to 100
            80..=89 => grade = 'B',  // Matches marks from 80 to 89
            70..=79 => grade = 'C',  // Matches marks from 70 to 79
            _ => grade = 'F',        // Default arm, matches anything not covered by previous patterns
        }
    }
    ```

    - **Description**: The `match` statement allows for more complex conditional logic based on patterns. Each pattern is compared against the value, and the first matching pattern's associated code is executed.
    - **Notes**:
        - `match` is also an expression, meaning it returns a value.
        - Patterns can be ranges (`90..=100`), specific values, or wildcards (`_`).
        - The wildcard `_` is a catch-all pattern that matches any value not covered by previous patterns. It ensures that the `match` expression is exhaustive.

---
