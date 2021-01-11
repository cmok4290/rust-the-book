# Common Programming Concepts

## Variables and Mutability
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
