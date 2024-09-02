 
---

# Primitive Data Types in Rust

## 1. Unsigned Integers
- **Types**: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
- **Range**: From `0` to `2^N - 1`, where `N` is the number of bits.

```rust
let unsigned_num: u8 = 5;
```

## 2. Signed Integers
- **Types**: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`
- **Range**: From `-2^(N-1)` to `2^(N-1) - 1`.

```rust
let signed_num: i8 = -5;
```

## 3. Floating-Point Numbers
- **Types**: `f32` (32-bit), `f64` (64-bit)
- **Default**: Floating-point literals are of type `f64` by default.

```rust
let float_num_32: f32 = 5.0; // 32-bit floating-point
let float_num_64: f64 = 10.0; // 64-bit floating-point
```

## 4. Boolean
- **Type**: `bool`
- **Values**: `true`, `false`

```rust
let is_active: bool = true;
```

## 5. Character
- **Type**: `char`
- **Supports**: Unicode scalar values, representing a wide range of characters.

```rust
let letter: char = 'a';
let emoji: char = '😊';
```

## 6. Platform-Specific Integers
- **Types**: `usize`, `isize`
- **Description**: Pointer-sized integers that depend on the architecture of the machine (32-bit or 64-bit).

```rust
let arch_1: usize = 5; // Unsigned pointer-sized integer
let arch_2: isize = -5; // Signed pointer-sized integer
```

## 7. Unit Type
- **Type**: `()`
- **Description**: Represents the absence of a value, used in functions that do not return a value.

```rust
let unit: () = (); // Represents "no value"
```

---
