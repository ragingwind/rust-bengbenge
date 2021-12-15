# bengbenge

> Simple inifity array for round-robin dns, beng, beng in rustlang. [Origin](https://github.com/ragingwind/node-bengbenge) is from node.

# Usese

```rust
use bengbenge::BengBenge;

fn main() {
    let mut bbe = BengBenge::new();

    bbe.append("192.168.0.1".to_string());
    bbe.append("192.168.0.2".to_string());

    println!("{}", bbe.next().unwrap());
    println!("{}", bbe.next().unwrap());
    println!("{}", bbe.next().unwrap());
    println!("{}", bbe.next().unwrap());
}
```

# License

MIT @ Jimmy Moon