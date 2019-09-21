## hello_exercism

### Step 1
- Go to Crate Root
```bash
echo 'hello_exercism = "0.1.3"' >> Cargo.toml
```

### Step 2
- Go to Crate Root
```rust
// vi src/main.rs
use hello_exercism;

fn main () {
    println!("{}",hello_exercism::hello());
    assert_eq!("Hello, World!", hello_exercism::hello());
}
```
