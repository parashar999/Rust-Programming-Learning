### Notes on Borrowing in Functions

#### 1. **Borrowing in Function Parameters**
   - **Borrowing** allows a function to access data without taking ownership of it. This enables multiple functions to work with the same data without duplicating or transferring ownership.
   - **Example**:
     ```rust
     fn borrow_reference(vec: &Vec<i32>) {
         // The function can read the data but cannot modify it.
     }
     ```
     - Here, `borrow_reference` takes an immutable reference to a `Vec<i32>`. The function can read the vector but cannot modify it.

#### 2. **Ownership Transfer in Functions**
   - If a function takes ownership of a value, the original variable is no longer valid after the function call.
   - **Example**:
     ```rust
     fn takes_ownership(vec: Vec<i32>) {
         // Ownership of vec is transferred to the function.
     }
     ```
     - After `takes_ownership` is called, the original `vec` cannot be used unless it is explicitly returned.

#### 3. **Passing Mutable References**
   - A **mutable reference** can be passed to a function when the data needs to be modified without transferring ownership.
   - **Example**:
     ```rust
     fn mutably_borrow(vec: &mut Vec<i32>) {
         vec.push(10); // Modifies the original vector.
     }
     ```
     - `mutably_borrow` takes a mutable reference and can change the contents of the vector.

#### 4. **Return Values and Ownership**
   - When a function creates and returns a value, it must transfer ownership of that value.
   - Returning a reference to a locally created value would result in a **dangling reference**, which is not allowed.
   - **Example**:
     ```rust
     fn take_n_give(mut vec: Vec<i32>) -> Vec<i32> {
         vec.push(10);
         vec // Ownership of vec is returned to the caller.
     }
     ```
     - Here, `take_n_give` takes ownership of `vec`, modifies it, and then returns it.

#### 5. **Function Usage in `main()`**
   - **Immutable Borrowing**:
     ```rust
     let vec_1: Vec<i32> = vec![1, 2, 3];
     let ref1: &Vec<i32> = &vec_1;
     borrow_reference(ref1); // Passes an immutable reference.
     ```
   - **Ownership Transfer**:
     ```rust
     take_ownership(vec_1); // Ownership of vec_1 is transferred.
     println!("vec 1 is: {:?}", vec_1); // This will not compile because vec_1's ownership was transferred.
     ```
   - **Mutable Borrowing**:
     ```rust
     let mut vec_2: Vec<i32> = vec![1, 2, 3];
     let ref2: &mut Vec<i32> = &mut vec_2;
     mutably_borrow(ref2); // Passes a mutable reference.
     ```

#### 6. **Shadowing for Efficiency**
   - Shadowing can be used to reintroduce a variable after its ownership has been transferred.
   - **Example**:
     ```rust
     let vec_1: Vec<i32> = take_n_give(vec_1);
     ```
     - This approach allows reuse of `vec_1` without creating a new variable, but it’s not always efficient.

#### 7. **Dangling References in Functions**
   - Attempting to return a reference to a locally created value leads to a dangling reference, which Rust prevents at compile time.
   - **Example** (will not compile):
     ```rust
     fn gives_ownership() -> &Vec<i32> {
         let vec1 = vec![4, 5, 6];
         &vec1 // vec1 will be dropped at the end of this function, leading to a dangling reference.
     }
     ```

#### 8. **Best Practices**
   - **Prefer Borrowing Over Ownership**: When a function doesn’t need ownership, prefer passing a reference (mutable or immutable) to avoid unnecessary memory allocation.
   - **Avoid Returning References**: If a function creates a value, return ownership rather than a reference to avoid dangling references.
   - **Use Shadowing Judiciously**: Shadowing can be helpful but should be used with care to avoid confusion and inefficiency.