# 3. Common Programming Concepts

## 3.1 Variables and Mutability
```
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
```

### Differences between Variables and Constants
`const MAX_POINTS: u32 = 100_000;`

### Shadowing
```
fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
}
```

## 3.2 Data Types
`let guess: u32 = "42".parse().expect("Not a number!");`

### Scalar Types
#### Integer Types
Table 3-1: Integer Types in Rust
Length | Signed | Unsigned
--- | --- | ---
8-bit | i8 | u8
16-bit | i16 | u16
32-bit | i32 | u32
64-bit | i64 | u64
128-bit | i128 | u128
arch | isize | usize

Table 3-2: Integer Literals in Rust
Number literals | Example
--- | ---
Decimal | 98_222
Hex | 0xff
Octal | 0o77
Binary | 0b1111_0000
Byte (u8 only) | b'A'

#### Floating-Point Types
```
fn main() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
}
```

#### Numeric Operations
```
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;
}
```

#### The Boolean Type
```
fn main() {
    let t = true;
    let f: bool = false; // with explicit type annotation
}
```

#### The Character Type
```
fn main() {
    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';
}
```

### Compound Types
#### The Tuple Type
```
fn main() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
}
```

```
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
}
```

#### The Array Type
```
fn main() {
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June",
        "July", "August", "September", "October", "November", "December"];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [3; 5]; // [3, 3, 3, 3, 3]
}
```

```
// Accessing Array Elements
fn main() {
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
}
```

