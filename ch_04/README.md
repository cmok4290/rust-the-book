# 4. Understanding Ownership

## 4.1 What is Ownership?

### Ownership Rules
- Each value in Rust has a variable that's called its _owner_.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.
