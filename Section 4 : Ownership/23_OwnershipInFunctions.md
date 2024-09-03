### Ownership in Functions and Memory Allocation

In Rust, when data is passed to functions, the behavior of ownership and memory allocation depends on whether the data is stored on the stack or the heap.

#### Key Points in the Provided Code:

1. **Ownership Transfer to Functions**:
   - When `vec_1` is passed to `takes_ownership`, ownership of `vec_1` is transferred to the function.
   - After the function call, `vec_1` is no longer valid, and attempting to use it will result in a compile-time error.
   - At the end of `takes_ownership`, the data that `vec_1` was pointing to in the heap is cleaned up.

   ```rust
   fn takes_ownership(vec: Vec<i32>) {
       // code here
   }
   ```

2. **Cloning to Prevent Ownership Loss**:
   - The method `.clone()` is used on `vec_2` before passing it to `take_ownership`.
   - Cloning creates a new instance of the vector in the heap, and the ownership of this new instance is transferred to the function.
   - The original `vec_2` remains valid after the function call.

   ```rust
   fn main() {
       let vec_2 = Vec::<i32>::from(vec![1, 2, 3]);
       take_ownership(vec_2.clone()); // Cloning prevents ownership loss
       print!("vec 2 is: {:?}", vec_2); // This is valid
   }
   ```

3. **Return Ownership from Functions**:
   - `gives_ownership` returns a `Vec<i32>`, transferring ownership of the newly created vector to the caller.
   - The returned vector is then owned by `vec_3`.

   ```rust
   fn gives_ownership() -> Vec<i32> {
       vec![4, 5, 6]
   }
   ```

4. **Simultaneous Take and Give**:
   - `take_n_give` takes ownership of the vector passed to it, performs operations, and then returns a new vector.
   - Ownership of the new vector is transferred to `vec_4`.

   ```rust
   fn take_n_give(vec: Vec<i32>) -> Vec<i32> {
       vec![4, 5, 6]
   }
   ```

5. **Primitive Types and Stack Allocation**:
   - The integer `x` is passed to `stack_function`. Since `i32` is a primitive type stored on the stack, it is copied rather than moved.
   - Ownership is not transferred, and `x` remains valid after the function call.

   ```rust
   fn stack_function(mut var: i32) {
       // code here
   }
   ```

