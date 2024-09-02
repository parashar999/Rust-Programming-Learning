# Variables and Constants

### 1. **Type Inference**:
   - Rust has a strong type inference system, so you don't always need to specify the type explicitly when declaring a variable. If you omit the type, Rust will infer it based on the value you assign.
   - Example:
     ```rust
     let x = 10; // Rust infers that x is of type i32
     let y = 10.0; // Rust infers that y is of type f64
     ```

### 2. **Immutable by Default**:
   - Variables in Rust are immutable by default, which means you cannot change their value after they are initialized unless you explicitly make them mutable using the `mut` keyword.
   - Example:
     ```rust
     let x = 10;
     x = 20; // Error: cannot assign twice to immutable variable `x`
     let mut y = 10;
     y = 20; // This works because `y` is mutable
     ```

### 3. **Constants**:
   - Constants are always immutable and must have their type explicitly specified. They can be declared in any scope, including the global scope.
   - Constants are evaluated at compile-time and must be set to a constant expression, not a value that could change at runtime.
   - Constants follow the naming convention of all-uppercase with underscores between words.
   - Example:
     ```rust
     const SECONDS_IN_A_MINUTE: u32 = 60;
     ```

### 4. **Shadowing**:
   - Shadowing allows you to reuse the same variable name in the same scope. Unlike `mut`, shadowing doesn’t modify the original variable but instead creates a new variable.
   - Shadowing can change the variable’s type if needed.
   - Example:
     ```rust
     let x = 5;
     let x = "Hello"; // The type of x has changed from i32 to &str
     ```

### 5. **Floating-Point Types**:
   - In your example, `let k:f32=10.0;` is actually defining a 32-bit floating-point number, not a 64-bit one.
   - Example:
     ```rust
     let a = 2.5; // a is inferred to be f64
     let b: f32 = 3.2; // b is explicitly set to f32
     ```

### 6. **Scope and Lifetimes**:
   - Rust enforces strict scope rules, so variables are only accessible within the scope they are defined. This ensures memory safety.
   - Example:
     ```rust
     {
         let z = 50; // z is only valid within this block
     }
     println!("{}", z); // Error: z is not found in this scope
     ```

### 7. **Performance Considerations**:
   - Constants are inlined at compile time, which can make them more efficient in certain scenarios compared to using mutable variables.
   - Variables, when mutable, might introduce some overhead due to possible memory reassignment.

### 8. **Mutable Constants**:
   - It’s worth noting that Rust doesn’t allow mutable constants. If you need a mutable value that behaves similarly to a constant, you should use a mutable variable with the `mut` keyword.

