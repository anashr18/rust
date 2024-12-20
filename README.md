# rust
Rust related..

### generic 
```rust
impl<T> Add<T> for MyValue
where
    T: Into<f64>,
{
    type Output = MyValue;

    fn add(self, rhs: T) -> MyValue {
        MyValue(self.0 + rhs.into())
    }
}
Let’s analyze it step by step:

impl<T>

Purpose: Declares that we are defining a generic implementation, where T is a generic parameter.
Usage: T can represent any type that satisfies the constraints specified in the where clause.
Add<T>

Purpose: Specifies that this implementation is for the Add trait, where the right-hand side operand (rhs) is of type T.
Add<T> allows MyValue to be added to values of type T (the type passed in as the generic parameter).
```
### Clap cli args
``` rust
cargo run --bin regex -- --pattern "Lifeless"
```